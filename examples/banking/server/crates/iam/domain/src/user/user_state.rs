use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{UserDisplayName, UserId, UserProfile, UserStateError, Username};

/// Stores the materialized state of a `User` aggregate.
#[aggregate_state(error = UserStateError)]
#[unique_constraints(entry(key = "username", values = username_values))]
pub struct UserState {
    id: UserId,
    profile: UserProfile,
}

impl UserState {
    /// Creates a new user state.
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            profile: UserProfile::Pending,
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

    /// Replaces the current profile.
    pub fn set_profile(&mut self, profile: UserProfile) {
        self.profile = profile;
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

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use super::{UserDisplayName, UserId, UserProfile, UserState, Username};

    #[test]
    fn pending_profile_has_no_username_unique_entry() {
        let state = UserState::new(UserId::new());

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("username"))
                .map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn ready_profile_returns_unique_entries_for_username() {
        let mut state = UserState::new(UserId::new());
        state.set_profile(UserProfile::Ready {
            username: Username::try_from("alice").expect("username should be valid"),
            display_name: UserDisplayName::try_from("Alice Example")
                .expect("display name should be valid"),
        });

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("username"))
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = UserId::new();
        let state = UserState::new(id);

        assert_eq!(state.id(), id);
    }
}
