use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::core::Email;
use crate::user::UserId;

use super::{UserIdentityId, UserIdentityProvider, UserIdentityStateError, UserIdentitySubject};

/// Stores the materialized state of a `UserIdentity` aggregate.
#[aggregate_state(error = UserIdentityStateError)]
#[unique_constraints(entry(key = "provider_subject", values = provider_subject_values))]
pub struct UserIdentityState {
    id: UserIdentityId,
    user_id: Option<UserId>,
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    email: Option<Email>,
}

impl UserIdentityState {
    /// Creates a new user-identity state.
    pub fn new(
        id: UserIdentityId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Self {
        Self {
            id,
            user_id: None,
            provider,
            subject,
            email,
        }
    }

    /// Returns the linked user identifier.
    pub fn user_id(&self) -> Option<&UserId> {
        self.user_id.as_ref()
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

    /// Replaces the current email snapshot.
    pub fn set_email(&mut self, email: Option<Email>) {
        self.email = email;
    }

    /// Links the identity to a user.
    pub fn set_user_id(&mut self, user_id: UserId) {
        self.user_id = Some(user_id);
    }
}

fn provider_subject_values(
    state: &UserIdentityState,
) -> Result<Option<UniqueValues>, UserIdentityStateError> {
    let provider = UniqueValuePart::try_from(state.provider().as_ref())?;
    let subject = UniqueValuePart::try_from(state.subject().as_ref())?;
    let value = UniqueValue::new(vec![provider, subject])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::core::Email;

    use super::{UserIdentityId, UserIdentityProvider, UserIdentityState, UserIdentitySubject};

    #[test]
    fn returns_unique_entries_for_provider_and_subject() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let state = UserIdentityState::new(
            UserIdentityId::new(&provider, &subject),
            provider,
            subject,
            Some(Email::try_from("alice@example.com").expect("email should be valid")),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("provider_subject"))
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let id = UserIdentityId::new(&provider, &subject);
        let state = UserIdentityState::new(id, provider, subject, None);

        assert_eq!(state.id(), id);
    }
}
