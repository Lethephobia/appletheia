use appletheia::application::command::{CommandFailureReaction, CommandOptions};
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::{
    CurrencyIssuanceSagaError, CurrencyIssuanceSagaSpec, CurrencyIssuanceSagaState,
    CurrencyIssuanceSagaStatus,
};
use crate::command::{
    AccountDepositCommand, CurrencyDefinitionDecreaseSupplyCommand,
    CurrencyDefinitionIncreaseSupplyCommand, CurrencyIssuanceCompleteCommand,
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
                    currency_definition_id,
                    destination_account_id,
                    amount,
                } => {
                    *instance.state_mut() = Some(CurrencyIssuanceSagaState::new(
                        *id,
                        *currency_definition_id,
                        *destination_account_id,
                        *amount,
                    ));

                    instance.append_command(
                        event,
                        &CurrencyDefinitionIncreaseSupplyCommand {
                            currency_definition_id: *currency_definition_id,
                            amount: *amount,
                        },
                        CommandOptions {
                            failure_reaction: {
                                let mut reaction = CommandFailureReaction::new();
                                reaction.push(
                                    &CurrencyIssuanceFailCommand {
                                        currency_issuance_id: *id,
                                    },
                                    CommandOptions::default(),
                                )?;
                                reaction
                            },
                            ..CommandOptions::default()
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
        } else if event.is_for_aggregate::<CurrencyDefinition>() {
            let currency_definition_event = event.try_into_domain_event::<CurrencyDefinition>()?;
            match currency_definition_event.payload() {
                CurrencyDefinitionEventPayload::SupplyIncreased { .. } => {
                    let (
                        destination_account_id,
                        amount,
                        currency_definition_id,
                        currency_issuance_id,
                    ) = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = CurrencyIssuanceSagaStatus::SupplyIncreased;
                        (
                            state.destination_account_id,
                            state.amount,
                            state.currency_definition_id,
                            state.currency_issuance_id,
                        )
                    };

                    instance.append_command(
                        event,
                        &AccountDepositCommand {
                            account_id: destination_account_id,
                            amount,
                        },
                        CommandOptions {
                            failure_reaction: {
                                let mut reaction = CommandFailureReaction::new();
                                reaction.push(
                                    &CurrencyDefinitionDecreaseSupplyCommand {
                                        currency_definition_id,
                                        amount,
                                    },
                                    CommandOptions {
                                        failure_reaction: {
                                            let mut reaction = CommandFailureReaction::new();
                                            reaction.push(
                                                &CurrencyIssuanceFailCommand {
                                                    currency_issuance_id,
                                                },
                                                CommandOptions::default(),
                                            )?;
                                            reaction
                                        },
                                        ..CommandOptions::default()
                                    },
                                )?;
                                reaction
                            },
                            ..CommandOptions::default()
                        },
                    )?;
                }
                CurrencyDefinitionEventPayload::SupplyDecreased { .. } => {
                    let currency_issuance_id = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = CurrencyIssuanceSagaStatus::SupplyDecreased;
                        state.currency_issuance_id
                    };

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id,
                        },
                        CommandOptions::default(),
                    )?;
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            if let AccountEventPayload::Deposited { .. } = account_event.payload() {
                let currency_issuance_id = {
                    let Some(state) = instance.state_mut().as_mut() else {
                        return Ok(());
                    };
                    state.status = CurrencyIssuanceSagaStatus::Deposited;
                    state.currency_issuance_id
                };

                instance.append_command(
                    event,
                    &CurrencyIssuanceCompleteCommand {
                        currency_issuance_id,
                    },
                    CommandOptions::default(),
                )?;
            }
        }

        Ok(())
    }
}
