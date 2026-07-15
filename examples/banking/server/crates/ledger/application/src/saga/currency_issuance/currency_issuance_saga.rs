use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{
    CurrencyIssuance, CurrencyIssuanceEventPayload, CurrencyIssuanceFailureReason,
};

use super::{
    CurrencyIssuanceSagaError, CurrencyIssuanceSagaSpec, CurrencyIssuanceSagaState,
    CurrencyIssuanceSagaStatus,
};
use crate::command::{
    AccountDepositCommand, CurrencyIssuanceCompleteCommand, CurrencyIssuanceFailCommand,
    CurrencySupplyCommitCommand, CurrencySupplyReleaseCommand, CurrencySupplyReserveCommand,
    MintSupplySyncCommand,
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
                    currency_id,
                    destination_account_id,
                    amount,
                    ..
                } => {
                    *instance.state_mut() = Some(CurrencyIssuanceSagaState::new(
                        issuance_event.aggregate_id(),
                        *currency_id,
                        *destination_account_id,
                        *amount,
                    ));

                    instance.append_command(
                        event,
                        &CurrencySupplyReserveCommand {
                            currency_id: *currency_id,
                            amount: *amount,
                        },
                    )?;
                }
                CurrencyIssuanceEventPayload::Completed => {
                    instance.state_required_mut()?.status = CurrencyIssuanceSagaStatus::Completed;
                    instance.succeed()
                }
                CurrencyIssuanceEventPayload::IssueRejected { .. } => {
                    instance.state_required_mut()?.status = CurrencyIssuanceSagaStatus::Failed;
                    instance.fail()
                }
                CurrencyIssuanceEventPayload::Failed { .. } => {
                    instance.state_required_mut()?.status = CurrencyIssuanceSagaStatus::Failed;
                    instance.fail()
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Currency>() {
            let currency_event = event.try_into_domain_event::<Currency>()?;
            match currency_event.payload() {
                CurrencyEventPayload::SupplyReserved { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_id = state.currency_id;
                    state.status = CurrencyIssuanceSagaStatus::MintSupplySyncRequested;

                    instance.append_command(event, &MintSupplySyncCommand { currency_id })?;
                }
                CurrencyEventPayload::SupplyReserveRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_issuance_id = state.currency_issuance_id;
                    state.status = CurrencyIssuanceSagaStatus::FailRequested;

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id,
                            reason: CurrencyIssuanceFailureReason::SupplyReserveRejected,
                        },
                    )?;
                }
                CurrencyEventPayload::MintSupplySynced { .. } => {
                    let state = instance.state_required_mut()?;
                    match state.status {
                        CurrencyIssuanceSagaStatus::MintSupplySyncRequested => {
                            let destination_account_id = state.destination_account_id;
                            let amount = state.amount;
                            state.status = CurrencyIssuanceSagaStatus::DepositRequested;

                            instance.append_command(
                                event,
                                &AccountDepositCommand {
                                    account_id: destination_account_id,
                                    amount,
                                },
                            )?;
                        }
                        CurrencyIssuanceSagaStatus::SupplyReleaseMintSupplySyncRequested => {
                            let currency_issuance_id = state.currency_issuance_id;
                            state.status = CurrencyIssuanceSagaStatus::FailRequested;

                            instance.append_command(
                                event,
                                &CurrencyIssuanceFailCommand {
                                    currency_issuance_id,
                                    reason: CurrencyIssuanceFailureReason::DepositRejected,
                                },
                            )?;
                        }
                        _ => {}
                    }
                }
                CurrencyEventPayload::SupplyCommitted { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_issuance_id = state.currency_issuance_id;
                    state.status = CurrencyIssuanceSagaStatus::CompleteRequested;

                    instance.append_command(
                        event,
                        &CurrencyIssuanceCompleteCommand {
                            currency_issuance_id,
                        },
                    )?;
                }
                CurrencyEventPayload::SupplyCommitRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_issuance_id = state.currency_issuance_id;
                    state.status = CurrencyIssuanceSagaStatus::FailRequested;

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id,
                            reason: CurrencyIssuanceFailureReason::SupplyCommitRejected,
                        },
                    )?;
                }
                CurrencyEventPayload::SupplyReleased { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_id = state.currency_id;
                    state.status = CurrencyIssuanceSagaStatus::SupplyReleaseMintSupplySyncRequested;

                    instance.append_command(event, &MintSupplySyncCommand { currency_id })?;
                }
                CurrencyEventPayload::SupplyReleaseRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_issuance_id = state.currency_issuance_id;
                    state.status = CurrencyIssuanceSagaStatus::FailRequested;

                    instance.append_command(
                        event,
                        &CurrencyIssuanceFailCommand {
                            currency_issuance_id,
                            reason: CurrencyIssuanceFailureReason::SupplyReleaseRejected,
                        },
                    )?;
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::Deposited { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_id = state.currency_id;
                    let amount = state.amount;
                    state.status = CurrencyIssuanceSagaStatus::SupplyCommitRequested;

                    instance.append_command(
                        event,
                        &CurrencySupplyCommitCommand {
                            currency_id,
                            amount,
                        },
                    )?;
                }
                AccountEventPayload::DepositRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    let currency_id = state.currency_id;
                    let amount = state.amount;
                    state.status = CurrencyIssuanceSagaStatus::SupplyReleaseRequested;

                    instance.append_command(
                        event,
                        &CurrencySupplyReleaseCommand {
                            currency_id,
                            amount,
                        },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
