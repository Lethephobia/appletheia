use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    OrganizationJoinRequestApproveRejectionReason, OrganizationJoinRequestCancelRejectionReason,
    OrganizationJoinRequestId, OrganizationJoinRequestRejectRejectionReason,
    OrganizationJoinRequestStateError, OrganizationJoinRequestSubmitRejectionReason,
};

/// Describes why an `OrganizationJoinRequest` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationJoinRequestId>),

    #[error(transparent)]
    State(#[from] OrganizationJoinRequestStateError),

    #[error("organization join request is already submitted")]
    AlreadySubmitted,
    #[error("organization join request submission rejected: {0:?}")]
    SubmitRejected(OrganizationJoinRequestSubmitRejectionReason),
    #[error("organization join request approval rejected: {0:?}")]
    ApproveRejected(OrganizationJoinRequestApproveRejectionReason),
    #[error("organization join request rejection rejected: {0:?}")]
    RejectRejected(OrganizationJoinRequestRejectRejectionReason),
    #[error("organization join request cancellation rejected: {0:?}")]
    CancelRejected(OrganizationJoinRequestCancelRejectionReason),
}
