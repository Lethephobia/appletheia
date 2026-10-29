use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{UserId, UserStateError};

/// Describes why a `User` aggregate operation failed.
#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<UserId>),

    #[error(transparent)]
    State(#[from] UserStateError),

    #[error("user is already created")]
    AlreadyCreated,

    #[error("user identity state is invalid")]
    InvalidIdentityState,

    #[error("user is inactive")]
    Inactive,

    #[error("user has been deleted")]
    Deleted,

    #[error("username is already taken")]
    UsernameAlreadyTaken,

    #[error("user identity was not found")]
    IdentityNotFound,

    #[error("identity is already linked to a user")]
    IdentityAlreadyLinked,

    #[error("user identity count limit has been reached")]
    IdentityLimitExceeded,
}
