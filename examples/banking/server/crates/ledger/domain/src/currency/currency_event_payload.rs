use appletheia::event_payload;

use crate::core::{CurrencyCode, CurrencyDecimals};
use crate::currency_registrar::CurrencyRegistrarId;

use super::{CurrencyDescription, CurrencyEventPayloadError, CurrencyLifecycleRejectionReason};

/// Represents events emitted by a Currency aggregate.
#[event_payload(error = CurrencyEventPayloadError)]
pub enum CurrencyEventPayload {
    Defined {
        currency_registrar_id: CurrencyRegistrarId,
        code: CurrencyCode,
        decimals: CurrencyDecimals,
        description: Option<CurrencyDescription>,
    },
    DescriptionChanged {
        description: Option<CurrencyDescription>,
    },
    Activated,
    ActivationRejected {
        reason: CurrencyLifecycleRejectionReason,
    },
    Deactivated,
    DeactivationRejected {
        reason: CurrencyLifecycleRejectionReason,
    },
}
