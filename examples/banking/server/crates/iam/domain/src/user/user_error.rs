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

    #[error("user profile is already ready")]
    ProfileAlreadyReady,

    #[error("user profile is not ready")]
    ProfileNotReady,
}
