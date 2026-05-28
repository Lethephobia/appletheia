use appletheia::domain::AggregateError;
use thiserror::Error;

use super::OrganizationJoinRequestId;

/// Describes why an `OrganizationJoinRequest` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationJoinRequestId>),

    #[error("organization join request is already submitted")]
    AlreadySubmitted,
}
