use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    OrganizationInvitationAcceptRejectionReason, OrganizationInvitationCancelRejectionReason,
    OrganizationInvitationDeclineRejectionReason, OrganizationInvitationId,
    OrganizationInvitationIssueRejectionReason, OrganizationInvitationStateError,
};

/// Describes why an `OrganizationInvitation` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationInvitationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationInvitationId>),

    #[error(transparent)]
    State(#[from] OrganizationInvitationStateError),

    #[error("organization invitation is already issued")]
    AlreadyIssued,
    #[error("organization invitation issue rejected: {0:?}")]
    IssueRejected(OrganizationInvitationIssueRejectionReason),
    #[error("organization invitation acceptance rejected: {0:?}")]
    AcceptRejected(OrganizationInvitationAcceptRejectionReason),
    #[error("organization invitation decline rejected: {0:?}")]
    DeclineRejected(OrganizationInvitationDeclineRejectionReason),
    #[error("organization invitation cancellation rejected: {0:?}")]
    CancelRejected(OrganizationInvitationCancelRejectionReason),
}
