use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{WithdrawalSagaError, WithdrawalSagaSpec, WithdrawalSagaState, WithdrawalSagaStatus};
use crate::command::{
    AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, WithdrawalCompleteCommand, WithdrawalFailCommand,
    WithdrawalSettlementExecuteCommand,
};

/// Coordinates the withdrawal flow.
pub struct WithdrawalSaga;

impl Saga for WithdrawalSaga {
    type Spec = WithdrawalSagaSpec;
    type Error = WithdrawalSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Withdrawal>() {
            let withdrawal_event = event.try_into_domain_event::<Withdrawal>()?;
            match withdrawal_event.payload() {
                WithdrawalEventPayload::Requested {
                    account_id, amount, ..
                } => {
                    *instance.state_mut() = Some(WithdrawalSagaState::new(
                        withdrawal_event.aggregate_id(),
                        *account_id,
                        *amount,
                    ));
                    instance.append_command(
                        event,
                        &AccountFundsReserveCommand {
                            account_id: *account_id,
                            amount: *amount,
                        },
                    )?;
                }
                WithdrawalEventPayload::SettlementExecuted { .. } => {
                    let state = instance.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    state.status = WithdrawalSagaStatus::ReservedFundsCommitRequested;
                    instance.append_command(
                        event,
                        &AccountReservedFundsCommitCommand { account_id, amount },
                    )?;
                }
                WithdrawalEventPayload::SettlementExecuteRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    state.status = WithdrawalSagaStatus::ReservedFundsReleaseRequested;
                    instance.append_command(
                        event,
                        &AccountReservedFundsReleaseCommand { account_id, amount },
                    )?;
                }
                WithdrawalEventPayload::Completed => {
                    instance.state_required_mut()?.status = WithdrawalSagaStatus::Completed;
                    instance.succeed();
                }
                WithdrawalEventPayload::Failed { .. } => {
                    instance.state_required_mut()?.status = WithdrawalSagaStatus::Failed;
                    instance.fail();
                }
                _ => {}
            }
            return Ok(());
        }

        if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::FundsReserved { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::SettlementExecuteRequested;
                    instance.append_command(
                        event,
                        &WithdrawalSettlementExecuteCommand { withdrawal_id },
                    )?;
                }
                AccountEventPayload::FundsReserveRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::FundsReserveRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsReleased { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::SettlementExecuteRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsReleaseRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::ReservedFundsReleaseRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsCommitted { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::CompleteRequested;
                    instance.append_command(event, &WithdrawalCompleteCommand { withdrawal_id })?;
                }
                AccountEventPayload::ReservedFundsCommitRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::ReservedFundsCommitRejected,
                        },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
