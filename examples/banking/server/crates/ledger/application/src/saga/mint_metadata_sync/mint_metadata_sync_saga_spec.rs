use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::MintMetadataSyncSagaState;

/// Declares the descriptor and state for the currency mint metadata sync saga.
pub struct MintMetadataSyncSagaSpec;

impl SagaSpec for MintMetadataSyncSagaSpec {
    type State = MintMetadataSyncSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("mint_metadata_sync"),
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
            EventSelector::new::<Currency>(CurrencyEventPayload::MINT_METADATA_SYNCED),
            EventSelector::new::<Currency>(CurrencyEventPayload::MINT_METADATA_SYNC_REJECTED),
        ]),
    );
}
