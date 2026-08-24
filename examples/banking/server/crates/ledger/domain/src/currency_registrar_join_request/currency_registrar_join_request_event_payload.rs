use appletheia::event_payload;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{
    CurrencyRegistrarJoinRequestApproveRejectionReason,
    CurrencyRegistrarJoinRequestCancelRejectionReason,
    CurrencyRegistrarJoinRequestEventPayloadError,
    CurrencyRegistrarJoinRequestRejectRejectionReason,
    CurrencyRegistrarJoinRequestSubmitRejectionReason,
};

/// Represents the domain events emitted by an `CurrencyRegistrarJoinRequest` aggregate.
#[event_payload(error = CurrencyRegistrarJoinRequestEventPayloadError)]
pub enum CurrencyRegistrarJoinRequestEventPayload {
    Submitted {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    SubmitRejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
        reason: CurrencyRegistrarJoinRequestSubmitRejectionReason,
    },
    Approved {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    ApproveRejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
        reason: CurrencyRegistrarJoinRequestApproveRejectionReason,
    },
    Rejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    RejectRejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
        reason: CurrencyRegistrarJoinRequestRejectRejectionReason,
    },
    Canceled {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    },
    CancelRejected {
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
        reason: CurrencyRegistrarJoinRequestCancelRejectionReason,
    },
}
