use appletheia::event_payload;

use super::{
    CurrencyRegistrarCreateRejectionReason, CurrencyRegistrarDescription,
    CurrencyRegistrarDisplayName, CurrencyRegistrarEventPayloadError, CurrencyRegistrarHandle,
    CurrencyRegistrarHandleChangeRejectionReason,
};

/// Represents events emitted by a CurrencyRegistrar aggregate.
#[event_payload(error = CurrencyRegistrarEventPayloadError)]
pub enum CurrencyRegistrarEventPayload {
    Created {
        handle: CurrencyRegistrarHandle,
        display_name: CurrencyRegistrarDisplayName,
        description: Option<CurrencyRegistrarDescription>,
    },
    CreateRejected {
        handle: CurrencyRegistrarHandle,
        display_name: CurrencyRegistrarDisplayName,
        description: Option<CurrencyRegistrarDescription>,
        reason: CurrencyRegistrarCreateRejectionReason,
    },
    HandleChanged {
        handle: CurrencyRegistrarHandle,
    },
    HandleChangeRejected {
        handle: CurrencyRegistrarHandle,
        reason: CurrencyRegistrarHandleChangeRejectionReason,
    },
    DisplayNameChanged {
        display_name: CurrencyRegistrarDisplayName,
    },
    DescriptionChanged {
        description: Option<CurrencyRegistrarDescription>,
    },
}
