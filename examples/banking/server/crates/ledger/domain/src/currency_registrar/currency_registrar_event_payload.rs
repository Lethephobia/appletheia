use appletheia::event_payload;

use super::{
    CurrencyRegistrarDescription, CurrencyRegistrarDisplayName, CurrencyRegistrarEventPayloadError,
    CurrencyRegistrarHandle,
};

/// Represents events emitted by a CurrencyRegistrar aggregate.
#[event_payload(error = CurrencyRegistrarEventPayloadError)]
pub enum CurrencyRegistrarEventPayload {
    Created {
        handle: CurrencyRegistrarHandle,
        display_name: CurrencyRegistrarDisplayName,
        description: Option<CurrencyRegistrarDescription>,
    },
    HandleChanged {
        handle: CurrencyRegistrarHandle,
    },
    DisplayNameChanged {
        display_name: CurrencyRegistrarDisplayName,
    },
    DescriptionChanged {
        description: Option<CurrencyRegistrarDescription>,
    },
}
