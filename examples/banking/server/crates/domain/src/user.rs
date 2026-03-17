mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_id_error;
mod user_state;
mod user_state_error;

pub use user_error::UserError;
pub use user_event_payload::UserEventPayload;
pub use user_event_payload_error::UserEventPayloadError;
pub use user_id::UserId;
pub use user_id_error::UserIdError;
pub use user_state::UserState;
pub use user_state_error::UserStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{Email, Username};

/// Represents the `User` aggregate root.
#[aggregate(type = "user", error = UserError)]
pub struct User {
    core: AggregateCore<UserState, UserEventPayload>,
}

impl User {
    /// Registers a new user.
    pub fn register(&mut self, email: Email, username: Username) -> Result<(), UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        self.append_event(UserEventPayload::Registered {
            id: UserId::new(),
            email,
            username,
        })
    }

    /// Changes the current email.
    pub fn change_email(&mut self, email: Email) -> Result<(), UserError> {
        if self.state_required()?.email().eq(&email) {
            return Ok(());
        }

        self.append_event(UserEventPayload::EmailChanged { email })
    }

    /// Changes the current username.
    pub fn change_username(&mut self, username: Username) -> Result<(), UserError> {
        if self.state_required()?.username().eq(&username) {
            return Ok(());
        }

        self.append_event(UserEventPayload::UsernameChanged { username })
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered {
                id,
                email,
                username,
            } => {
                self.set_state(Some(UserState::new(*id, email.clone(), username.clone())));
            }
            UserEventPayload::EmailChanged { email } => {
                self.state_required_mut()?.set_email(email.clone());
            }
            UserEventPayload::UsernameChanged { username } => {
                self.state_required_mut()?.set_username(username.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

    use crate::core::{Email, Username};

    use super::{User, UserEventPayload, UserId};

    #[test]
    fn register_initializes_state_and_records_event() {
        let email = Email::try_from("alice@example.com").expect("email should be valid");
        let username = Username::try_from("Alice Example").expect("username should be valid");
        let mut user = User::default();

        user.register(email.clone(), username.clone())
            .expect("registration should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state.id(),
            user.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.email(), &email);
        assert_eq!(state.username(), &username);
        assert_eq!(user.uncommitted_events().len(), 1);
        assert_eq!(
            user.uncommitted_events()[0].payload().name(),
            UserEventPayload::REGISTERED
        );
    }

    #[test]
    fn changing_to_same_values_is_a_no_op() {
        let email = Email::try_from("alice@example.com").expect("email should be valid");
        let username = Username::try_from("Alice Example").expect("username should be valid");
        let mut user = User::default();
        user.register(email.clone(), username.clone())
            .expect("registration should succeed");

        user.change_email(email)
            .expect("no-op email change should succeed");
        user.change_username(username)
            .expect("no-op username change should succeed");

        assert_eq!(user.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_methods_append_events_and_update_state() {
        let initial_email = Email::try_from("alice@example.com").expect("email should be valid");
        let initial_name = Username::try_from("Alice").expect("username should be valid");
        let changed_email = Email::try_from("alice@bank.example").expect("email should be valid");
        let changed_name = Username::try_from("Alice Example").expect("username should be valid");
        let mut user = User::default();
        user.register(initial_email, initial_name)
            .expect("registration should succeed");

        user.change_email(changed_email.clone())
            .expect("email change should succeed");
        user.change_username(changed_name.clone())
            .expect("username change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.email(), &changed_email);
        assert_eq!(state.username(), &changed_name);
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::EMAIL_CHANGED
        );
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::USERNAME_CHANGED
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = UserId::new();
        let registered = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            UserEventPayload::Registered {
                id,
                email: Email::try_from("alice@example.com").expect("email should be valid"),
                username: Username::try_from("Alice").expect("username should be valid"),
            },
        );
        let email_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            UserEventPayload::EmailChanged {
                email: Email::try_from("alice@bank.example").expect("email should be valid"),
            },
        );
        let mut user = User::default();

        user.replay_events(vec![registered, email_changed], None)
            .expect("events should replay");

        let state = user.state().expect("state should exist");
        assert_eq!(state.email().value(), "alice@bank.example");
        assert_eq!(user.version().value(), 2);
        assert!(user.uncommitted_events().is_empty());
    }

    #[test]
    fn register_rejects_already_registered_user() {
        let mut user = User::default();
        user.register(
            Email::try_from("alice@example.com").expect("email should be valid"),
            Username::try_from("Alice").expect("username should be valid"),
        )
        .expect("registration should succeed");

        let error = user
            .register(
                Email::try_from("alice2@example.com").expect("email should be valid"),
                Username::try_from("Alice 2").expect("username should be valid"),
            )
            .expect_err("duplicate registration should fail");

        assert!(matches!(error, super::UserError::AlreadyRegistered));
    }
}
