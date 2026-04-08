use appletheia::domain::AggregateError;
use thiserror::Error;

use super::UserId;

/// Describes why a `User` aggregate operation failed.
#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<UserId>),

    #[error("user is already registered")]
    AlreadyRegistered,

    #[error("user is inactive")]
    Inactive,

    #[error("user is removed")]
    Removed,

    #[error("user identity is already linked")]
    IdentityAlreadyLinked,

    #[error("user identity was not found")]
    IdentityNotFound,

    #[error("user profile is already ready")]
    ProfileAlreadyReady,

    #[error("user profile is not ready")]
    ProfileNotReady,

    #[error("user profile state is invalid")]
    InvalidProfileState,

    #[error("user identity state is invalid")]
    InvalidIdentityState,
}
