use appletheia::event_payload;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{
    CurrencyRegistrarInvitationAcceptRejectionReason,
    CurrencyRegistrarInvitationCancelRejectionReason,
    CurrencyRegistrarInvitationDeclineRejectionReason,
    CurrencyRegistrarInvitationEventPayloadError, CurrencyRegistrarInvitationExpiresAt,
    CurrencyRegistrarInvitationIssueRejectionReason, CurrencyRegistrarInvitationIssuer,
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
    IssueRejected {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
        issuer: CurrencyRegistrarInvitationIssuer,
        expires_at: CurrencyRegistrarInvitationExpiresAt,
        reason: CurrencyRegistrarInvitationIssueRejectionReason,
    },
    Accepted {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
    AcceptRejected {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
        reason: CurrencyRegistrarInvitationAcceptRejectionReason,
    },
    Declined {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
    DeclineRejected {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
        reason: CurrencyRegistrarInvitationDeclineRejectionReason,
    },
    Canceled {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
    },
    CancelRejected {
        currency_registrar_id: CurrencyRegistrarId,
        invitee_id: UserId,
        reason: CurrencyRegistrarInvitationCancelRejectionReason,
    },
}
