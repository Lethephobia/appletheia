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

    #[error("user identity state is invalid")]
    InvalidIdentityState,

    #[error("user organization membership state is invalid")]
    InvalidOrganizationMembershipState,
}
