use appletheia::event_payload;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{
    CurrencyRegistrarInvitationEventPayloadError, CurrencyRegistrarInvitationExpiresAt,
    CurrencyRegistrarInvitationIssuer,
};

/// Represents the domain events emitted by an `CurrencyRegistrarInvitation` aggregate.
#[event_payload(error = CurrencyRegistrarInvitationEventPayloadError)]
pub enum CurrencyRegistrarInvitationEventPayload {
    Issued {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
        issuer: CurrencyRegistrarInvitationIssuer,
        expires_at: CurrencyRegistrarInvitationExpiresAt,
    },
    Accepted {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
    Declined {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
    Canceled {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
}
