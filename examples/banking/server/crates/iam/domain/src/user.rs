mod user_display_name;
mod user_display_name_error;
mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_state;
mod user_state_error;
mod username;
mod username_error;

pub use user_display_name::UserDisplayName;
pub use user_display_name_error::UserDisplayNameError;
pub use user_error::UserError;
pub use user_event_payload::UserEventPayload;
pub use user_event_payload_error::UserEventPayloadError;
pub use user_id::UserId;
pub use user_state::UserState;
pub use user_state_error::UserStateError;
pub use username::Username;
pub use username_error::UsernameError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `User` aggregate root.
#[aggregate(type = "user", error = UserError)]
pub struct User {
    core: AggregateCore<UserState, UserEventPayload>,
}

impl User {
    /// Registers a new user.
    pub fn register(&mut self) -> Result<(), UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        self.append_event(UserEventPayload::Registered {
            id: UserId::new(),
            username: Username::new_random(),
        })
    }

    /// Changes the current username.
    pub fn change_username(&mut self, username: Username) -> Result<(), UserError> {
        if self.state_required()?.username().eq(&username) {
            return Ok(());
        }

        self.append_event(UserEventPayload::UsernameChanged { username })
    }

    /// Changes the current display name.
    pub fn change_display_name(
        &mut self,
        display_name: Option<UserDisplayName>,
    ) -> Result<(), UserError> {
        if self.state_required()?.display_name() == display_name.as_ref() {
            return Ok(());
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered { id, username } => {
                self.set_state(Some(UserState::new(*id, username.clone())));
            }
            UserEventPayload::UsernameChanged { username } => {
                self.state_required_mut()?.set_username(username.clone());
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?
                    .set_display_name(display_name.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

    use super::{User, UserDisplayName, UserEventPayload, UserId, Username};

    #[test]
    fn register_initializes_state_and_records_event() {
        let mut user = User::default();

        user.register().expect("registration should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state.id(),
            user.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.username().value().chars().count(), 32);
        assert_eq!(state.display_name(), None);
        assert_eq!(user.uncommitted_events().len(), 1);
        assert_eq!(
            user.uncommitted_events()[0].payload().name(),
            UserEventPayload::REGISTERED
        );
    }

    #[test]
    fn changing_to_same_username_is_a_no_op() {
        let mut user = User::default();
        user.register().expect("registration should succeed");
        let username = user.state().expect("state should exist").username().clone();

        user.change_username(username)
            .expect("no-op username change should succeed");

        assert_eq!(user.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_username_appends_event_and_updates_state() {
        let changed_name = Username::try_from("alice_example").expect("username should be valid");
        let mut user = User::default();
        user.register().expect("registration should succeed");

        user.change_username(changed_name.clone())
            .expect("username change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.username(), &changed_name);
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::USERNAME_CHANGED
        );
    }

    #[test]
    fn change_display_name_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register().expect("registration should succeed");

        let display_name =
            Some(UserDisplayName::try_from("Alice Example").expect("display name should be valid"));
        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.display_name(), display_name.as_ref());
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::DISPLAY_NAME_CHANGED
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
                username: Username::try_from("alice").expect("username should be valid"),
            },
        );
        let username_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            UserEventPayload::UsernameChanged {
                username: Username::try_from("alice_example").expect("username should be valid"),
            },
        );
        let display_name_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            UserEventPayload::DisplayNameChanged {
                display_name: Some(
                    UserDisplayName::try_from("Alice Example")
                        .expect("display name should be valid"),
                ),
            },
        );
        let mut user = User::default();

        user.replay_events(
            vec![registered, username_changed, display_name_changed],
            None,
        )
        .expect("events should replay");

        let state = user.state().expect("state should exist");
        assert_eq!(state.username().value(), "alice_example");
        assert_eq!(
            state
                .display_name()
                .expect("display name should exist")
                .value(),
            "Alice Example"
        );
        assert_eq!(user.version().value(), 3);
        assert!(user.uncommitted_events().is_empty());
    }

    #[test]
    fn register_rejects_already_registered_user() {
        let mut user = User::default();
        user.register().expect("registration should succeed");

        let error = user
            .register()
            .expect_err("duplicate registration should fail");

        assert!(matches!(error, super::UserError::AlreadyRegistered));
    }
}
