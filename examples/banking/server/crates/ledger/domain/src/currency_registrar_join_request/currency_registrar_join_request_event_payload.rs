use appletheia::event_payload;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::CurrencyRegistrarJoinRequestEventPayloadError;

/// Represents the domain events emitted by an `CurrencyRegistrarJoinRequest` aggregate.
#[event_payload(error = CurrencyRegistrarJoinRequestEventPayloadError)]
pub enum CurrencyRegistrarJoinRequestEventPayload {
    Submitted {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    Approved {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    Rejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    Canceled {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
}
