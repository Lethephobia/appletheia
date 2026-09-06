use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload, DepositFailureReason};

use super::{DepositSagaError, DepositSagaSpec, DepositSagaState, DepositSagaStep};
use crate::command::{AccountDepositCommand, DepositCompleteCommand, DepositFailCommand};

/// Coordinates the deposit flow.
pub struct DepositSaga;

impl Saga for DepositSaga {
    type Spec = DepositSagaSpec;
    type Step = DepositSagaStep;
    type Error = DepositSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Deposit>() {
            let deposit_event = event.try_into_domain_event::<Deposit>()?;
            match deposit_event.payload() {
                DepositEventPayload::SettlementVerified {
                    account_id, amount, ..
                } if step.is_none() => {
                    *instance.state_mut() = Some(DepositSagaState::new(
                        deposit_event.aggregate_id(),
                        *account_id,
                        *amount,
                    ));
                    instance.append_command(
                        CausationId::from(event.event_id),
                        DepositSagaStep::Deposit,
                        &AccountDepositCommand {
                            account_id: *account_id,
                            amount: *amount,
                        },
                    )?;
                }
                DepositEventPayload::Completed if step == Some(DepositSagaStep::Complete) => {
                    instance.succeed();
                }
                DepositEventPayload::Failed { .. } if step == Some(DepositSagaStep::Fail) => {
                    instance.fail();
                }
                _ => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::Deposited { .. } if step == Some(DepositSagaStep::Deposit) => {
                    let state = instance.state_required_mut()?;
                    let deposit_id = state.deposit_id;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        DepositSagaStep::Complete,
                        &DepositCompleteCommand { deposit_id },
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
        if step == DepositSagaStep::Deposit {
            let state = instance.state_required_mut()?;
            let deposit_id = state.deposit_id;
            instance.append_command(
                CausationId::from(failure.failure_id),
                DepositSagaStep::Fail,
                &DepositFailCommand {
                    deposit_id,
                    reason: DepositFailureReason::AccountDepositRejected,
                },
            )?;
        } else if matches!(step, DepositSagaStep::Complete | DepositSagaStep::Fail) {
            instance.fail();
        }
        Ok(())
    }
}
