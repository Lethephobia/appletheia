mod user_identity_provider;
mod user_identity_provider_error;
mod user_identity_subject;
mod user_identity_subject_error;

pub use user_identity_provider::UserIdentityProvider;
pub use user_identity_provider_error::UserIdentityProviderError;
pub use user_identity_subject::UserIdentitySubject;
pub use user_identity_subject_error::UserIdentitySubjectError;

use banking_shared_kernel_domain::contact::Email;
use serde::{Deserialize, Serialize};

/// Represents an external identity linked to a `User`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserIdentity {
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
