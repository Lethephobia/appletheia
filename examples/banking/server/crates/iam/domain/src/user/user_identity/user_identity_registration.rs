use serde::{Deserialize, Serialize};

use crate::core::Email;

use super::{UserIdentityProvider, UserIdentitySubject};

/// Describes an external identity to register or link to a `User`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserIdentityRegistration {
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
