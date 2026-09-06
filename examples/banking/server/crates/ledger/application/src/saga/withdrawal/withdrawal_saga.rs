use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{WithdrawalSagaError, WithdrawalSagaSpec, WithdrawalSagaState, WithdrawalSagaStep};
use crate::command::{
    AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, WithdrawalCompleteCommand, WithdrawalFailCommand,
    WithdrawalSettlementExecuteCommand,
};

/// Coordinates the withdrawal flow.
pub struct WithdrawalSaga;

impl Saga for WithdrawalSaga {
    type Spec = WithdrawalSagaSpec;
    type Step = WithdrawalSagaStep;
    type Error = WithdrawalSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Withdrawal>() {
            let withdrawal_event = event.try_into_domain_event::<Withdrawal>()?;
            match withdrawal_event.payload() {
                WithdrawalEventPayload::Requested {
                    account_id, amount, ..
                } if step.is_none() => {
                    *instance.state_mut() = Some(WithdrawalSagaState::new(
                        withdrawal_event.aggregate_id(),
                        *account_id,
                        *amount,
                    ));
                    instance.append_command(
                        CausationId::from(event.event_id),
                        WithdrawalSagaStep::ReserveFunds,
                        &AccountFundsReserveCommand {
                            account_id: *account_id,
                            amount: *amount,
                        },
                    )?;
                }
                WithdrawalEventPayload::SettlementExecuted { .. }
                    if step == Some(WithdrawalSagaStep::ExecuteSettlement) =>
                {
                    let state = instance.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    instance.append_command(
                        CausationId::from(event.event_id),
                        WithdrawalSagaStep::CommitFunds,
                        &AccountReservedFundsCommitCommand { account_id, amount },
                    )?;
                }
                WithdrawalEventPayload::Completed if step == Some(WithdrawalSagaStep::Complete) => {
                    instance.succeed();
                }
                WithdrawalEventPayload::Failed { .. } if step == Some(WithdrawalSagaStep::Fail) => {
                    instance.fail();
                }
                _ => {}
            }
            return Ok(());
        }

        if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::FundsReserved { .. }
                    if step == Some(WithdrawalSagaStep::ReserveFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        CausationId::from(event.event_id),
                        WithdrawalSagaStep::ExecuteSettlement,
                        &WithdrawalSettlementExecuteCommand { withdrawal_id },
                    )?;
                }
                AccountEventPayload::ReservedFundsReleased { .. }
                    if step == Some(WithdrawalSagaStep::ReleaseFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        CausationId::from(event.event_id),
                        WithdrawalSagaStep::Fail,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::SettlementExecuteRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsCommitted { .. }
                    if step == Some(WithdrawalSagaStep::CommitFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        CausationId::from(event.event_id),
                        WithdrawalSagaStep::Complete,
                        &WithdrawalCompleteCommand { withdrawal_id },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        failure: &CommandFailureEnvelope,
        step: Self::Step,
    ) -> Result<(), Self::Error> {
        if step == WithdrawalSagaStep::ReserveFunds {
            self.append_fail_after_failure(
                instance,
                failure,
                WithdrawalFailureReason::FundsReserveRejected,
            )?;
        } else if step == WithdrawalSagaStep::ExecuteSettlement {
            let state = instance.state_required_mut()?;
            let account_id = state.account_id;
            let amount = state.amount;
            instance.append_command(
                CausationId::from(failure.failure_id),
                WithdrawalSagaStep::ReleaseFunds,
                &AccountReservedFundsReleaseCommand { account_id, amount },
            )?;
        } else if step == WithdrawalSagaStep::ReleaseFunds {
            self.append_fail_after_failure(
                instance,
                failure,
                WithdrawalFailureReason::ReservedFundsReleaseRejected,
            )?;
        } else if step == WithdrawalSagaStep::CommitFunds {
            self.append_fail_after_failure(
                instance,
                failure,
                WithdrawalFailureReason::ReservedFundsCommitRejected,
            )?;
        } else if matches!(
            step,
            WithdrawalSagaStep::Complete | WithdrawalSagaStep::Fail
        ) {
            instance.fail();
        }
        Ok(())
    }
}

impl WithdrawalSaga {
    fn append_fail_after_failure(
        &self,
        instance: &mut SagaInstance<WithdrawalSagaState, WithdrawalSagaStep>,
        failure: &CommandFailureEnvelope,
        reason: WithdrawalFailureReason,
    ) -> Result<(), WithdrawalSagaError> {
        let state = instance.state_required_mut()?;
        let withdrawal_id = state.withdrawal_id;
        instance.append_command(
            CausationId::from(failure.failure_id),
            WithdrawalSagaStep::Fail,
            &WithdrawalFailCommand {
                withdrawal_id,
                reason,
            },
        )?;
        Ok(())
    }
}
