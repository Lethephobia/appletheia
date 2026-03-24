use serde::{Deserialize, Serialize};

use crate::core::Email;

use super::{UserIdentityProvider, UserIdentitySubject};

/// Represents an external identity linked to a `User`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserIdentity {
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    email: Option<Email>,
}

impl UserIdentity {
    /// Creates a linked external identity.
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

    /// Returns the current email snapshot.
    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    /// Returns whether the identity matches the provider / subject pair.
    pub fn matches(&self, provider: &UserIdentityProvider, subject: &UserIdentitySubject) -> bool {
        self.provider().eq(provider) && self.subject().eq(subject)
    }

    /// Replaces the current email snapshot.
    pub fn set_email(&mut self, email: Option<Email>) {
        self.email = email;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Email;

    use super::{UserIdentity, UserIdentityProvider, UserIdentitySubject};

    #[test]
    fn matches_provider_and_subject_pair() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let identity = UserIdentity::new(
            provider.clone(),
            subject.clone(),
            Some(Email::try_from("alice@example.com").expect("email should be valid")),
        );

        assert!(identity.matches(&provider, &subject));
    }

    #[test]
    fn exposes_email_snapshot() {
        let identity = UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            Some(Email::try_from("alice@example.com").expect("email should be valid")),
        );

        assert_eq!(
            identity.email().expect("email should exist").value(),
            "alice@example.com"
        );
    }
}
