use appletheia::domain::AggregateError;
use thiserror::Error;

use super::RoleId;

/// Describes why a `Role` aggregate operation failed.
#[derive(Debug, Error)]
pub enum RoleError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<RoleId>),

    #[error("role is already created")]
    AlreadyCreated,
}
