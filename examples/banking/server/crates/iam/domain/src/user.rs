mod user_display_name;
mod user_display_name_error;
mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_identity;
mod user_identity_provider;
mod user_identity_provider_error;
mod user_identity_subject;
mod user_identity_subject_error;
mod user_profile;
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
pub use user_identity::UserIdentity;
pub use user_identity_provider::UserIdentityProvider;
pub use user_identity_provider_error::UserIdentityProviderError;
pub use user_identity_subject::UserIdentitySubject;
pub use user_identity_subject_error::UserIdentitySubjectError;
pub use user_profile::UserProfile;
pub use user_state::UserState;
pub use user_state_error::UserStateError;
pub use username::Username;
pub use username_error::UsernameError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::Email;

/// Represents the `User` aggregate root.
#[aggregate(type = "user", error = UserError)]
pub struct User {
    core: AggregateCore<UserState, UserEventPayload>,
}

impl User {
    /// Registers a new user with an initial external identity.
    pub fn register(&mut self, identity: UserIdentity) -> Result<(), UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        self.append_event(UserEventPayload::Registered {
            id: UserId::new(),
            identity,
        })
    }

    /// Marks the profile as ready with explicit profile values.
    pub fn ready_profile(
        &mut self,
        username: Username,
        display_name: UserDisplayName,
    ) -> Result<(), UserError> {
        match self.state_required()?.profile() {
            UserProfile::Pending => {}
            UserProfile::Ready {
                username: current_username,
                display_name: current_display_name,
            } if current_username.eq(&username) && current_display_name.eq(&display_name) => {
                return Ok(());
            }
            UserProfile::Ready { .. } => {
                return Err(UserError::ProfileAlreadyReady);
            }
        }

        self.append_event(UserEventPayload::ProfileReadied {
            username,
            display_name,
        })
    }

    /// Changes the current username.
    pub fn change_username(&mut self, username: Username) -> Result<(), UserError> {
        let Some(current_username) = self.state_required()?.username() else {
            return Err(UserError::ProfileNotReady);
        };

        if current_username.eq(&username) {
            return Ok(());
        }

        self.append_event(UserEventPayload::UsernameChanged { username })
    }

    /// Changes the current display name.
    pub fn change_display_name(&mut self, display_name: UserDisplayName) -> Result<(), UserError> {
        let Some(current_display_name) = self.state_required()?.display_name() else {
            return Err(UserError::ProfileNotReady);
        };

        if current_display_name.eq(&display_name) {
            return Ok(());
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })
    }

    /// Links an additional external identity.
    pub fn link_identity(&mut self, identity: UserIdentity) -> Result<(), UserError> {
        if let Some(current_identity) = self
            .state_required()?
            .identity(identity.provider(), identity.subject())
        {
            if current_identity.eq(&identity) {
                return Ok(());
            }

            return Err(UserError::IdentityAlreadyLinked);
        }

        self.append_event(UserEventPayload::IdentityLinked { identity })
    }

    /// Changes the email snapshot for a linked identity.
    pub fn change_identity_email(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<(), UserError> {
        let Some(identity) = self.state_required()?.identity(&provider, &subject) else {
            return Err(UserError::IdentityNotFound);
        };

        if identity.email() == email.as_ref() {
            return Ok(());
        }

        self.append_event(UserEventPayload::IdentityEmailChanged {
            provider,
            subject,
            email,
        })
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered { id, identity } => {
                self.set_state(Some(UserState::new(*id, identity.clone())));
            }
            UserEventPayload::ProfileReadied {
                username,
                display_name,
            } => {
                self.state_required_mut()?.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name: display_name.clone(),
                };
            }
            UserEventPayload::UsernameChanged { username } => {
                let state = self.state_required_mut()?;
                let display_name = state
                    .display_name()
                    .cloned()
                    .ok_or(UserError::ProfileNotReady)?;
                state.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name,
                };
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                let state = self.state_required_mut()?;
                let username = state
                    .username()
                    .cloned()
                    .ok_or(UserError::ProfileNotReady)?;
                state.profile = UserProfile::Ready {
                    username,
                    display_name: display_name.clone(),
                };
            }
            UserEventPayload::IdentityLinked { identity } => {
                self.state_required_mut()?.identities.push(identity.clone());
            }
            UserEventPayload::IdentityEmailChanged {
                provider,
                subject,
                email,
            } => {
                let state = self.state_required_mut()?;
                let Some(identity) = state
                    .identities
                    .iter_mut()
                    .find(|identity| identity.matches(provider, subject))
                else {
                    return Err(UserError::IdentityNotFound);
                };
                identity.set_email(email.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

    use crate::core::Email;

    use super::{
        User, UserDisplayName, UserEventPayload, UserId, UserIdentity, UserIdentityProvider,
        UserIdentitySubject, UserProfile, Username,
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
    fn register_initializes_state_and_records_event() {
        let mut user = User::default();

        user.register(identity())
            .expect("registration should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state.id(),
            user.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.profile(), &UserProfile::Pending);
        assert_eq!(state.username(), None);
        assert_eq!(state.display_name(), None);
        assert_eq!(state.identities().len(), 1);
        assert_eq!(
            state.identities()[0]
                .email()
                .expect("email should exist")
                .value(),
            "alice@example.com"
        );
        assert_eq!(user.uncommitted_events().len(), 1);
        assert_eq!(
            user.uncommitted_events()[0].payload().name(),
            UserEventPayload::REGISTERED
        );
    }

    #[test]
    fn readying_to_same_profile_is_a_no_op() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let username = Username::try_from("alice").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        user.ready_profile(username.clone(), display_name.clone())
            .expect("profile ready should succeed");

        user.ready_profile(username, display_name)
            .expect("no-op profile ready should succeed");

        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn ready_profile_rejects_already_ready_user_with_different_values() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
        )
        .expect("profile ready should succeed");

        let error = user
            .ready_profile(
                Username::try_from("alice_example").expect("username should be valid"),
                UserDisplayName::try_from("Alice Updated").expect("display name should be valid"),
            )
            .expect_err("readying an already-ready profile should fail");

        assert!(matches!(error, super::UserError::ProfileAlreadyReady));
    }

    #[test]
    fn ready_profile_appends_event_and_updates_state() {
        let username = Username::try_from("alice_example").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        user.ready_profile(username.clone(), display_name.clone())
            .expect("profile ready should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state.profile(),
            &UserProfile::Ready {
                username: username.clone(),
                display_name: display_name.clone(),
            }
        );
        assert_eq!(state.username(), Some(&username));
        assert_eq!(state.display_name(), Some(&display_name));
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::PROFILE_READIED
        );
    }

    #[test]
    fn change_username_rejects_pending_profile() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        let error = user
            .change_username(Username::try_from("alice").expect("username should be valid"))
            .expect_err("pending profile should reject username changes");

        assert!(matches!(error, super::UserError::ProfileNotReady));
    }

    #[test]
    fn change_username_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
        )
        .expect("profile ready should succeed");
        let username = Username::try_from("alice_example").expect("username should be valid");

        user.change_username(username.clone())
            .expect("username change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.username(), Some(&username));
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::USERNAME_CHANGED
        );
    }

    #[test]
    fn change_display_name_rejects_pending_profile() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        let error = user
            .change_display_name(
                UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            )
            .expect_err("pending profile should reject display name changes");

        assert!(matches!(error, super::UserError::ProfileNotReady));
    }

    #[test]
    fn change_display_name_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
        )
        .expect("profile ready should succeed");
        let display_name =
            UserDisplayName::try_from("Alice Updated").expect("display name should be valid");

        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.display_name(), Some(&display_name));
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::DISPLAY_NAME_CHANGED
        );
    }

    #[test]
    fn link_identity_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let linked_identity = UserIdentity::new(
            UserIdentityProvider::try_from("https://login.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-456").expect("subject should be valid"),
            None,
        );

        user.link_identity(linked_identity.clone())
            .expect("identity link should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(state.identities().len(), 2);
        assert!(
            state
                .identity(linked_identity.provider(), linked_identity.subject())
                .is_some()
        );
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::IDENTITY_LINKED
        );
    }

    #[test]
    fn linking_same_identity_is_a_no_op() {
        let mut user = User::default();
        let identity = identity();
        user.register(identity.clone())
            .expect("registration should succeed");

        user.link_identity(identity)
            .expect("same identity link should succeed");

        assert_eq!(user.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_identity_email_rejects_unknown_identity() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        let error = user
            .change_identity_email(
                UserIdentityProvider::try_from("https://login.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-456").expect("subject should be valid"),
                Some(Email::try_from("other@example.com").expect("email should be valid")),
            )
            .expect_err("unknown identity should fail");

        assert!(matches!(error, super::UserError::IdentityNotFound));
    }

    #[test]
    fn change_identity_email_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let email = Some(Email::try_from("alice@bank.example").expect("email should be valid"));

        user.change_identity_email(provider.clone(), subject.clone(), email.clone())
            .expect("identity email change should succeed");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state
                .identity(&provider, &subject)
                .expect("identity should exist")
                .email(),
            email.as_ref()
        );
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::IDENTITY_EMAIL_CHANGED
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
                identity: identity(),
            },
        );
        let profile_readied = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            UserEventPayload::ProfileReadied {
                username: Username::try_from("alice_example").expect("username should be valid"),
                display_name: UserDisplayName::try_from("Alice Example")
                    .expect("display name should be valid"),
            },
        );
        let identity_linked = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            UserEventPayload::IdentityLinked {
                identity: UserIdentity::new(
                    UserIdentityProvider::try_from("https://login.example.com")
                        .expect("provider should be valid"),
                    UserIdentitySubject::try_from("user-456").expect("subject should be valid"),
                    None,
                ),
            },
        );
        let username_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(4).expect("version should be valid"),
            UserEventPayload::UsernameChanged {
                username: Username::try_from("alice_updated").expect("username should be valid"),
            },
        );
        let display_name_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(5).expect("version should be valid"),
            UserEventPayload::DisplayNameChanged {
                display_name: UserDisplayName::try_from("Alice Updated")
                    .expect("display name should be valid"),
            },
        );
        let identity_email_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(6).expect("version should be valid"),
            UserEventPayload::IdentityEmailChanged {
                provider: UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                subject: UserIdentitySubject::try_from("user-123")
                    .expect("subject should be valid"),
                email: Some(Email::try_from("alice@bank.example").expect("email should be valid")),
            },
        );
        let mut user = User::default();

        user.replay_events(
            vec![
                registered,
                profile_readied,
                identity_linked,
                username_changed,
                display_name_changed,
                identity_email_changed,
            ],
            None,
        )
        .expect("events should replay");

        let state = user.state().expect("state should exist");
        assert_eq!(
            state.username().expect("username should exist").value(),
            "alice_updated"
        );
        assert_eq!(
            state
                .display_name()
                .expect("display name should exist")
                .value(),
            "Alice Updated"
        );
        assert_eq!(state.identities().len(), 2);
        assert_eq!(
            state.identities()[0]
                .email()
                .expect("email should exist")
                .value(),
            "alice@bank.example"
        );
        assert_eq!(user.version().value(), 6);
        assert!(user.uncommitted_events().is_empty());
    }

    #[test]
    fn register_rejects_already_registered_user() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        let error = user
            .register(identity())
            .expect_err("duplicate registration should fail");

        assert!(matches!(error, super::UserError::AlreadyRegistered));
    }
}
