use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload, DepositFailureReason};

use super::{DepositSagaError, DepositSagaSpec, DepositSagaState, DepositSagaStatus};
use crate::command::{AccountDepositCommand, DepositCompleteCommand, DepositFailCommand};

/// Coordinates the deposit flow.
pub struct DepositSaga;

impl Saga for DepositSaga {
    type Spec = DepositSagaSpec;
    type Error = DepositSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Deposit>() {
            let deposit_event = event.try_into_domain_event::<Deposit>()?;
            match deposit_event.payload() {
                DepositEventPayload::SettlementVerified {
                    account_id, amount, ..
                } => {
                    *instance.state_mut() = Some(DepositSagaState::new(
                        deposit_event.aggregate_id(),
                        *account_id,
                        *amount,
                    ));
                    instance.append_command(
                        event,
                        &AccountDepositCommand {
                            account_id: *account_id,
                            amount: *amount,
                        },
                    )?;
                }
                DepositEventPayload::Completed => {
                    instance.state_required_mut()?.status = DepositSagaStatus::Completed;
                    instance.succeed();
                }
                DepositEventPayload::Failed { .. } => {
                    instance.state_required_mut()?.status = DepositSagaStatus::Failed;
                    instance.fail();
                }
                _ => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::Deposited { .. } => {
                    let state = instance.state_required_mut()?;
                    let deposit_id = state.deposit_id;
                    state.status = DepositSagaStatus::CompleteRequested;

                    instance.append_command(event, &DepositCompleteCommand { deposit_id })?;
                }
                AccountEventPayload::DepositRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let deposit_id = state.deposit_id;
                    state.status = DepositSagaStatus::FailRequested;

                    instance.append_command(
                        event,
                        &DepositFailCommand {
                            deposit_id,
                            reason: DepositFailureReason::AccountDepositRejected,
                        },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
