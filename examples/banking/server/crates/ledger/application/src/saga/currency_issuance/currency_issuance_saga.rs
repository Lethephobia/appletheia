use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::{CurrencyIssuanceSagaError, CurrencyIssuanceSagaSpec, CurrencyIssuanceSagaState};
use crate::command::{
    AccountDepositCommand, AccountDepositContext, CurrencyDefinitionIncreaseSupplyCommand,
    CurrencyDefinitionIncreaseSupplyContext, CurrencyIssuanceCompleteCommand,
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
        if event.aggregate_type.value() == CurrencyIssuance::TYPE.value() {
            let issuance_event = event.try_into_domain_event::<CurrencyIssuance>()?;
            match issuance_event.payload() {
                CurrencyIssuanceEventPayload::Issued {
                    id,
                    currency_definition_id,
                    destination_account_id,
                    amount,
                } => {
                    let next_state = instance
                        .state_mut()
                        .get_or_insert_with(CurrencyIssuanceSagaState::default);
                    next_state.currency_definition_id = Some(*currency_definition_id);
                    next_state.destination_account_id = Some(*destination_account_id);
                    next_state.amount = Some(*amount);
                    next_state.currency_issuance_id = Some(*id);

                    instance.append_command(
                        event,
                        &CurrencyDefinitionIncreaseSupplyCommand {
                            currency_definition_id: *currency_definition_id,
                            amount: *amount,
                            context: CurrencyDefinitionIncreaseSupplyContext::Issuance {
                                currency_issuance_id: *id,
                            },
                        },
                    )?;
                }
                CurrencyIssuanceEventPayload::Completed => instance.succeed(),
                CurrencyIssuanceEventPayload::Failed => instance.fail(),
            }

            return Ok(());
        }

        if event.aggregate_type.value() == CurrencyDefinition::TYPE.value() {
            let currency_definition_event = event.try_into_domain_event::<CurrencyDefinition>()?;
            match currency_definition_event.payload() {
                CurrencyDefinitionEventPayload::SupplyIncreased { .. } => {
                    let Some(state) = instance.state.as_ref() else {
                        return Ok(());
                    };

                    instance.append_command(
                        event,
                        &AccountDepositCommand {
                            account_id: state
                                .destination_account_id
                                .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                            amount: state
                                .amount
                                .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                            context: AccountDepositContext::Issuance {
                                currency_issuance_id: state
                                    .currency_issuance_id
                                    .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                                currency_definition_id: state
                                    .currency_definition_id
                                    .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                            },
                        },
                    )?;
                }
                CurrencyDefinitionEventPayload::SupplyDecreased { .. } => {
                    let Some(state) = instance.state.as_ref() else {
                        return Ok(());
                    };

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id: state
                                .currency_issuance_id
                                .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                        },
                    )?;
                }
                _ => {}
            }

            return Ok(());
        }

        if event.aggregate_type.value() == Account::TYPE.value() {
            let account_event = event.try_into_domain_event::<Account>()?;
            if let AccountEventPayload::Deposited { .. } = account_event.payload() {
                let Some(state) = instance.state.as_ref() else {
                    return Ok(());
                };

                instance.append_command(
                    event,
                    &CurrencyIssuanceCompleteCommand {
                        currency_issuance_id: state
                            .currency_issuance_id
                            .ok_or(CurrencyIssuanceSagaError::IncompleteState)?,
                    },
                )?;
            }
        }

        Ok(())
    }
}
