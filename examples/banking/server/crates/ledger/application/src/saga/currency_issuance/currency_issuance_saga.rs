use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::{
    CurrencyIssuanceSagaError, CurrencyIssuanceSagaSpec, CurrencyIssuanceSagaState,
    CurrencyIssuanceSagaStatus,
};
use crate::command::{
    AccountDepositCommand, CurrencyIncreaseSupplyCommand, CurrencyIssuanceCompleteCommand,
    CurrencyIssuanceFailCommand,
};

/// Coordinates the currency issuance flow.
pub struct CurrencyIssuanceSaga;

impl Saga for CurrencyIssuanceSaga {
    type Spec = CurrencyIssuanceSagaSpec;
    type Error = CurrencyIssuanceSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<CurrencyIssuance>() {
            let issuance_event = event.try_into_domain_event::<CurrencyIssuance>()?;
            match issuance_event.payload() {
                CurrencyIssuanceEventPayload::Issued {
                    id,
                    currency_id,
                    destination_account_id,
                    amount,
                } => {
                    *instance.state_mut() = Some(CurrencyIssuanceSagaState::new(
                        *id,
                        *currency_id,
                        *destination_account_id,
                        *amount,
                    ));

                    instance.append_command(
                        event,
                        &CurrencyIncreaseSupplyCommand {
                            currency_id: *currency_id,
                            amount: *amount,
                        },
                    )?;
                }
                CurrencyIssuanceEventPayload::Completed => {
                    if let Some(state) = instance.state_mut().as_mut() {
                        state.status = CurrencyIssuanceSagaStatus::Completed;
                    }
                    instance.succeed()
                }
                CurrencyIssuanceEventPayload::Failed => {
                    if let Some(state) = instance.state_mut().as_mut() {
                        state.status = CurrencyIssuanceSagaStatus::Failed;
                    }
                    instance.fail()
                }
            }

            return Ok(());
        } else if event.is_for_aggregate::<Currency>() {
            let currency_event = event.try_into_domain_event::<Currency>()?;
            match currency_event.payload() {
                CurrencyEventPayload::SupplyIncreased { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = CurrencyIssuanceSagaStatus::SupplyIncreased;
                    let destination_account_id = state.destination_account_id;
                    let amount = state.amount;

                    instance.append_command(
                        event,
                        &AccountDepositCommand {
                            account_id: destination_account_id,
                            amount,
                        },
                    )?;
                }
                CurrencyEventPayload::SupplyDecreased { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = CurrencyIssuanceSagaStatus::SupplyDecreased;
                    let currency_issuance_id = state.currency_issuance_id;

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id,
                        },
                    )?;
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            if let AccountEventPayload::Deposited { .. } = account_event.payload() {
                let state = instance.state_required_mut()?;
                state.status = CurrencyIssuanceSagaStatus::Deposited;
                let currency_issuance_id = state.currency_issuance_id;

                instance.append_command(
                    event,
                    &CurrencyIssuanceCompleteCommand {
                        currency_issuance_id,
                    },
                )?;
            }
        }

        Ok(())
    }
}
