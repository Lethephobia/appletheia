pub mod core;
pub mod user;
pub mod user_identity;

pub use core::{Email, EmailError};
pub use user::{
    User, UserDisplayName, UserDisplayNameError, UserError, UserEventPayload,
    UserEventPayloadError, UserId, UserProfile, UserState, UserStateError, Username, UsernameError,
};
pub use user_identity::{
    UserIdentity, UserIdentityError, UserIdentityEventPayload, UserIdentityEventPayloadError,
    UserIdentityId, UserIdentityIdError, UserIdentityProvider, UserIdentityProviderError,
    UserIdentityState, UserIdentityStateError, UserIdentitySubject, UserIdentitySubjectError,
};
