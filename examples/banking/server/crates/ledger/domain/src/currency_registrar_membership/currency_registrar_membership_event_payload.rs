use appletheia::event_payload;
use banking_iam_domain::UserId;

use super::{
    CurrencyRegistrarMembershipCreateRejectionReason, CurrencyRegistrarMembershipEventPayloadError,
    CurrencyRegistrarMembershipRemoveRejectionReason,
};
use crate::currency_registrar::CurrencyRegistrarId;

/// Represents events emitted by a CurrencyRegistrarMembership aggregate.
#[event_payload(error = CurrencyRegistrarMembershipEventPayloadError)]
pub enum CurrencyRegistrarMembershipEventPayload {
    Created {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    },
    CreateRejected {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
        reason: CurrencyRegistrarMembershipCreateRejectionReason,
    },
    Removed {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    },
    RemoveRejected {
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
        reason: CurrencyRegistrarMembershipRemoveRejectionReason,
    },
}
