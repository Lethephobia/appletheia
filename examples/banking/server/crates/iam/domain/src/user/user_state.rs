use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{UserId, UserIdentity, UserProfile, UserStateError, UserStatus, Username};

/// Stores the materialized state of a `User` aggregate.
#[aggregate_state(error = UserStateError)]
#[unique_constraints(
    entry(key = "username", values = username_values),
    entry(key = "provider_subject", values = provider_subject_values)
)]
pub struct UserState {
    pub(super) id: UserId,
    pub(super) status: UserStatus,
    pub(super) username: Option<Username>,
    pub(super) profile: Option<UserProfile>,
    pub(super) identities: Vec<UserIdentity>,
}

impl UserState {
    /// Creates a new user state.
    pub(super) fn new(id: UserId, identity: UserIdentity) -> Self {
        Self {
            id,
            status: UserStatus::Active,
            username: None,
            profile: None,
            identities: vec![identity],
        }
    }
}

fn username_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let Some(username) = state.username.as_ref() else {
        return Ok(None);
    };

    let part = UniqueValuePart::try_from(username.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

fn provider_subject_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    if state.status.is_removed() || state.identities.is_empty() {
        return Ok(None);
    }

    let values = state
        .identities
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
    use appletheia::domain::{UniqueConstraints, UniqueKey, UniqueValues};

    use crate::{
        UserDisplayName, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
        UserPictureRef, UserPictureUrl, UserProfile, UserState, UserStatus, Username, core::Email,
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
    fn pending_username_has_no_unique_entry() {
        let state = UserState::new(UserId::new(), identity());

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UserState::USERNAME_KEY).map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn configured_username_returns_unique_entries() {
        let mut state = UserState::new(UserId::new(), identity());
        state.username = Some(Username::try_from("alice").expect("username should be valid"));
        state.profile = Some(UserProfile::new(
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            None,
            Some(UserPictureRef::external_url(
                UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                    .expect("picture URL should be valid"),
            )),
        ));

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
    fn removed_state_has_no_unique_entries() {
        let mut state = UserState::new(UserId::new(), identity());
        state.status = UserStatus::Removed;
        state.username = Some(Username::try_from("alice").expect("username should be valid"));

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UserState::USERNAME_KEY).map(UniqueValues::len),
            None
        );
        assert_eq!(
            entries
                .get(UserState::PROVIDER_SUBJECT_KEY)
                .map(UniqueValues::len),
            None
        );
    }
}
