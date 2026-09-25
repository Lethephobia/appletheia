use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    UserBioChangeRejectionReason, UserDisplayNameChangeRejectionReason, UserId,
    UserIdentityEmailChangeRejectionReason, UserIdentityLinkRejectionReason,
    UserPictureChangeRejectionReason, UserStateError, UserStatusRejectionReason,
    UserUsernameChangeRejectionReason,
};

/// Describes why a `User` aggregate operation failed.
#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<UserId>),

    #[error(transparent)]
    State(#[from] UserStateError),

    #[error("user is already registered")]
    AlreadyRegistered,

    #[error("user identity state is invalid")]
    InvalidIdentityState,
    #[error("user identity link rejected: {0:?}")]
    IdentityLinkRejected(UserIdentityLinkRejectionReason),
    #[error("user identity email change rejected: {0:?}")]
    IdentityEmailChangeRejected(UserIdentityEmailChangeRejectionReason),
    #[error("user username change rejected: {0:?}")]
    UsernameChangeRejected(UserUsernameChangeRejectionReason),
    #[error("user display name change rejected: {0:?}")]
    DisplayNameChangeRejected(UserDisplayNameChangeRejectionReason),
    #[error("user bio change rejected: {0:?}")]
    BioChangeRejected(UserBioChangeRejectionReason),
    #[error("user picture change rejected: {0:?}")]
    PictureChangeRejected(UserPictureChangeRejectionReason),
    #[error("user activation rejected: {0:?}")]
    ActivateRejected(UserStatusRejectionReason),
    #[error("user deactivation rejected: {0:?}")]
    DeactivateRejected(UserStatusRejectionReason),
    #[error("user removal rejected: {0:?}")]
    RemoveRejected(UserStatusRejectionReason),
}
