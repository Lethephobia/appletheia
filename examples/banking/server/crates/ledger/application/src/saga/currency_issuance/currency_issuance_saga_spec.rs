use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::CurrencyIssuanceSagaState;

/// Declares the descriptor and state for the currency issuance saga.
pub struct CurrencyIssuanceSagaSpec;

impl SagaSpec for CurrencyIssuanceSagaSpec {
    type State = CurrencyIssuanceSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance"),
        SagaStartEvents::new(&[EventSelector::new::<CurrencyIssuance>(
            CurrencyIssuanceEventPayload::ISSUED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::ISSUE_REJECTED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_INCREASED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSITED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSIT_REJECTED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_DECREASED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_DECREASE_REJECTED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::COMPLETED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
