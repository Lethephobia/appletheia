use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{UserDisplayName, UserId, UserStateError, Username};

/// Stores the materialized state of a `User` aggregate.
#[aggregate_state(error = UserStateError)]
#[unique_constraints(entry(key = "username", values = username_values))]
pub struct UserState {
    id: UserId,
    username: Username,
    display_name: Option<UserDisplayName>,
}

impl UserState {
    /// Creates a new user state.
    pub fn new(id: UserId, username: Username) -> Self {
        Self {
            id,
            username,
            display_name: None,
        }
    }

    /// Returns the current username.
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// Returns the current display name.
    pub fn display_name(&self) -> Option<&UserDisplayName> {
        self.display_name.as_ref()
    }

    /// Replaces the current username.
    pub fn set_username(&mut self, username: Username) {
        self.username = username;
    }

    /// Replaces the current display name.
    pub fn set_display_name(&mut self, display_name: Option<UserDisplayName>) {
        self.display_name = display_name;
    }
}

fn username_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    let part = UniqueValuePart::try_from(state.username().as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use super::{UserId, UserState, Username};

    #[test]
    fn returns_unique_entries_for_username() {
        let state = UserState::new(
            UserId::new(),
            Username::try_from("alice").expect("username should be valid"),
        );

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
        let state = UserState::new(
            id,
            Username::try_from("alice").expect("username should be valid"),
        );

        assert_eq!(state.id(), id);
    }
}
