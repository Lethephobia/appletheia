use banking_shared_kernel_domain::contact::Email;
use serde::{Deserialize, Serialize};

use super::{UserIdentityProvider, UserIdentitySubject};

/// Stores identity data carried by user events.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserIdentityData {
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    email: Option<Email>,
}

impl UserIdentityData {
    /// Creates user identity data.
    pub fn new(
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Self {
        Self {
            provider,
            subject,
            email,
        }
    }

    /// Returns the provider identifier.
    pub fn provider(&self) -> &UserIdentityProvider {
        &self.provider
    }

    /// Returns the provider subject.
    pub fn subject(&self) -> &UserIdentitySubject {
        &self.subject
    }

    /// Returns the email snapshot.
    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    /// Returns whether the identity data matches the provider / subject pair.
    pub fn matches(&self, provider: &UserIdentityProvider, subject: &UserIdentitySubject) -> bool {
        self.provider().eq(provider) && self.subject().eq(subject)
    }
}
