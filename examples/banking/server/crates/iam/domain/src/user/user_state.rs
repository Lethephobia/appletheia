use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{
    UserDisplayName, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject, UserProfile,
    UserStateError, Username,
};

/// Stores the materialized state of a `User` aggregate.
#[aggregate_state(error = UserStateError)]
#[unique_constraints(
    entry(key = "username", values = username_values),
    entry(key = "provider_subject", values = provider_subject_values)
)]
pub struct UserState {
    pub(super) id: UserId,
    pub(super) profile: UserProfile,
    pub(super) identities: Vec<UserIdentity>,
}

impl UserState {
    /// Creates a new user state.
    pub(super) fn new(id: UserId, identity: UserIdentity) -> Self {
        Self {
            id,
            profile: UserProfile::Pending,
            identities: vec![identity],
        }
    }

    /// Returns the current profile.
    pub fn profile(&self) -> &UserProfile {
        &self.profile
    }

    /// Returns the current username.
    pub fn username(&self) -> Option<&Username> {
        self.profile.username()
    }

    /// Returns the current display name.
    pub fn display_name(&self) -> Option<&UserDisplayName> {
        self.profile.display_name()
    }

    /// Returns the linked external identities.
    pub fn identities(&self) -> &[UserIdentity] {
        &self.identities
    }

    /// Returns a linked identity by provider and subject.
    pub fn identity(
        &self,
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
    ) -> Option<&UserIdentity> {
        self.identities
            .iter()
            .find(|identity| identity.matches(provider, subject))
    }
}

fn username_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    let Some(username) = state.username() else {
        return Ok(None);
    };

    let part = UniqueValuePart::try_from(username.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

fn provider_subject_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    if state.identities().is_empty() {
        return Ok(None);
    }

    let values = state
        .identities()
        .iter()
        .map(|identity| {
            let provider = UniqueValuePart::try_from(identity.provider().as_ref())?;
            let subject = UniqueValuePart::try_from(identity.subject().as_ref())?;
            UniqueValue::new(vec![provider, subject]).map_err(UserStateError::from)
        })
        .collect::<Result<Vec<_>, UserStateError>>()?;
    let values = UniqueValues::new(values)?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::core::Email;

    use super::{
        UserDisplayName, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
        UserProfile, UserState, Username,
    };

    fn identity() -> UserIdentity {
        UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            Some(Email::try_from("alice@example.com").expect("email should be valid")),
        )
    }

    #[test]
    fn pending_profile_has_no_username_unique_entry() {
        let state = UserState::new(UserId::new(), identity());

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UserState::USERNAME_KEY).map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn ready_profile_returns_unique_entries_for_username() {
        let mut state = UserState::new(UserId::new(), identity());
        state.profile = UserProfile::Ready {
            username: Username::try_from("alice").expect("username should be valid"),
            display_name: UserDisplayName::try_from("Alice Example")
                .expect("display name should be valid"),
        };

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UserState::USERNAME_KEY).map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn identities_return_unique_entries_for_provider_subject() {
        let mut state = UserState::new(UserId::new(), identity());
        state.identities.push(UserIdentity::new(
            UserIdentityProvider::try_from("https://login.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-456").expect("subject should be valid"),
            None,
        ));

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UserState::PROVIDER_SUBJECT_KEY)
                .map(UniqueValues::len),
            Some(2)
        );
    }

    #[test]
    fn exposes_generated_unique_key_constants() {
        assert_eq!(UserState::USERNAME_KEY, UniqueKey::new("username"));
        assert_eq!(
            UserState::PROVIDER_SUBJECT_KEY,
            UniqueKey::new("provider_subject")
        );
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = UserId::new();
        let state = UserState::new(id, identity());

        assert_eq!(state.id(), id);
    }
}
