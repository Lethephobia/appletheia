use appletheia::domain::AggregateError;
use thiserror::Error;

use super::UserIdentityId;

/// Describes why a `UserIdentity` aggregate operation failed.
#[derive(Debug, Error)]
pub enum UserIdentityError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<UserIdentityId>),

    #[error("user identity is already created")]
    AlreadyCreated,

    #[error("user identity is already linked to a user")]
    AlreadyLinkedToUser,
}
