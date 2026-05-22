use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyMintAccountCreationSagaState;

const EVENT_SELECTORS: [EventSelector; 3] = [
    EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
    EventSelector::new::<Currency>(CurrencyEventPayload::MINT_ACCOUNT_RECORDED),
    EventSelector::new::<Currency>(CurrencyEventPayload::MINT_ACCOUNT_RECORD_REJECTED),
];

/// Declares the descriptor and state for the currency mint account creation saga.
pub struct CurrencyMintAccountCreationSagaSpec;

impl SagaSpec for CurrencyMintAccountCreationSagaSpec {
    type State = CurrencyMintAccountCreationSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_mint_account_creation"),
        SagaStartEvents::new(&[EventSelector::new::<Currency>(
            CurrencyEventPayload::DEFINED,
        )]),
        Subscription::AnyOf(&EVENT_SELECTORS),
    );
}
