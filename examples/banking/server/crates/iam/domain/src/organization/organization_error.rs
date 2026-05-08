use appletheia::domain::AggregateError;
use thiserror::Error;

use super::OrganizationId;

/// Describes why an `Organization` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationId>),

    #[error("organization is already created")]
    AlreadyCreated,
}
