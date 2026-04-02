pub mod core;
pub mod user;

pub use core::{Email, EmailError};
pub use user::{
    User, UserBio, UserBioError, UserDisplayName, UserDisplayNameError, UserError,
    UserEventPayload, UserEventPayloadError, UserId, UserIdentity, UserIdentityProvider,
    UserIdentityProviderError, UserIdentitySubject, UserIdentitySubjectError, UserProfile,
    UserState, UserStateError, UserStatus, Username, UsernameError,
};
