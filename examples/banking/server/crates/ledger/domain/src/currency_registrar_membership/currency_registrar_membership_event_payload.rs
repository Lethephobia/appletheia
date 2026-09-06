use appletheia::event_payload;
use banking_iam_domain::UserId;

use super::CurrencyRegistrarMembershipEventPayloadError;
use crate::currency_registrar::CurrencyRegistrarId;

/// Represents events emitted by a CurrencyRegistrarMembership aggregate.
#[event_payload(error = CurrencyRegistrarMembershipEventPayloadError)]
pub enum CurrencyRegistrarMembershipEventPayload {
    Created {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    },
    Removed {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    },
}
