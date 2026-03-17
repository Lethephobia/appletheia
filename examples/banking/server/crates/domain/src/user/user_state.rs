use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::core::{Email, Username};

use super::{UserId, UserStateError};

/// Stores the materialized state of a `User` aggregate.
#[aggregate_state(error = UserStateError)]
#[unique_constraints(entry(key = "email", values = email_values))]
pub struct UserState {
    id: UserId,
    email: Email,
    username: Username,
}

impl UserState {
    /// Creates a new user state.
    pub fn new(id: UserId, email: Email, username: Username) -> Self {
        Self {
            id,
            email,
            username,
        }
    }

    /// Returns the current email.
    pub fn email(&self) -> &Email {
        &self.email
    }

    /// Returns the current username.
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// Replaces the current email.
    pub fn set_email(&mut self, email: Email) {
        self.email = email;
    }

    /// Replaces the current username.
    pub fn set_username(&mut self, username: Username) {
        self.username = username;
    }
}

fn email_values(state: &UserState) -> Result<Option<UniqueValues>, UserStateError> {
    let part = UniqueValuePart::try_from(state.email().as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::core::{Email, Username};

    use super::{UserId, UserState};

    #[test]
    fn returns_unique_entries_for_email() {
        let state = UserState::new(
            UserId::new(),
            Email::try_from("alice@example.com").expect("email should be valid"),
            Username::try_from("Alice").expect("username should be valid"),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("email")).map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = UserId::new();
        let state = UserState::new(
            id,
            Email::try_from("alice@example.com").expect("email should be valid"),
            Username::try_from("Alice").expect("username should be valid"),
        );

        assert_eq!(state.id(), id);
    }
}
