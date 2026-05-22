use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyMintAccountMetadataSyncSagaState;

/// Declares the descriptor and state for the currency mint account metadata sync saga.
pub struct CurrencyMintAccountMetadataSyncSagaSpec;

impl SagaSpec for CurrencyMintAccountMetadataSyncSagaSpec {
    type State = CurrencyMintAccountMetadataSyncSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_mint_account_metadata_sync"),
        SagaStartEvents::new(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::IMAGE_CHANGED),
        ]),
        Subscription::AnyOf(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::IMAGE_CHANGED),
        ]),
    );
}
