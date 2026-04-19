mod user_bio;
mod user_bio_error;
mod user_display_name;
mod user_display_name_error;
mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_identity;
mod user_profile;
mod user_state;
mod user_state_error;
mod user_status;
mod username;
mod username_error;

pub use user_bio::UserBio;
pub use user_bio_error::UserBioError;
pub use user_display_name::UserDisplayName;
pub use user_display_name_error::UserDisplayNameError;
pub use user_error::UserError;
pub use user_event_payload::UserEventPayload;
pub use user_event_payload_error::UserEventPayloadError;
pub use user_id::UserId;
pub use user_identity::{
    UserIdentity, UserIdentityProvider, UserIdentityProviderError, UserIdentitySubject,
    UserIdentitySubjectError,
};
pub use user_profile::UserProfile;
pub use user_state::UserState;
pub use user_state_error::UserStateError;
pub use user_status::UserStatus;
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
    /// Returns the current user status.
    pub fn status(&self) -> Result<UserStatus, UserError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the user is active.
    pub fn is_active(&self) -> Result<bool, UserError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the user is inactive.
    pub fn is_inactive(&self) -> Result<bool, UserError> {
        Ok(self.state_required()?.status.is_inactive())
    }

    /// Returns whether the user is removed.
    pub fn is_removed(&self) -> Result<bool, UserError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Returns the current profile.
    pub fn profile(&self) -> Result<&UserProfile, UserError> {
        Ok(&self.state_required()?.profile)
    }

    /// Returns the current username.
    pub fn username(&self) -> Result<Option<&Username>, UserError> {
        Ok(self.state_required()?.profile.username())
    }

    /// Returns the current display name.
    pub fn display_name(&self) -> Result<Option<&UserDisplayName>, UserError> {
        Ok(self.state_required()?.profile.display_name())
    }

    /// Returns the current bio.
    pub fn bio(&self) -> Result<Option<&UserBio>, UserError> {
        Ok(self.state_required()?.profile.bio())
    }

    /// Returns the linked external identities.
    pub fn identities(&self) -> Result<&[UserIdentity], UserError> {
        Ok(&self.state_required()?.identities)
    }

    /// Returns a linked identity by provider and subject.
    pub fn identity(
        &self,
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
    ) -> Result<Option<&UserIdentity>, UserError> {
        Ok(self
            .state_required()?
            .identities
            .iter()
            .find(|identity| identity.matches(provider, subject)))
    }

    /// Registers a new user with an initial external identity.
    pub fn register(&mut self, identity: UserIdentity) -> Result<(), UserError> {
        self.ensure_not_registered()?;
        let id = UserId::new();
        self.append_event(UserEventPayload::Registered { id, identity })
    }

    /// Marks the profile as ready with explicit profile values.
    pub fn ready_profile(
        &mut self,
        username: Username,
        display_name: UserDisplayName,
        bio: Option<UserBio>,
    ) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if let UserProfile::Ready { .. } = &self.state_required()?.profile {
            return Err(UserError::ProfileAlreadyReady);
        }

        self.append_event(UserEventPayload::ProfileReadied {
            username,
            display_name,
            bio,
        })
    }

    /// Changes the current username.
    pub fn change_username(&mut self, username: Username) -> Result<(), UserError> {
        self.ensure_active_status()?;

        let Some(current_username) = self.state_required()?.profile.username() else {
            return Err(UserError::ProfileNotReady);
        };

        if current_username.eq(&username) {
            return Ok(());
        }

        self.append_event(UserEventPayload::UsernameChanged { username })
    }

    /// Changes the current display name.
    pub fn change_display_name(&mut self, display_name: UserDisplayName) -> Result<(), UserError> {
        self.ensure_active_status()?;

        let Some(current_display_name) = self.state_required()?.profile.display_name() else {
            return Err(UserError::ProfileNotReady);
        };

        if current_display_name.eq(&display_name) {
            return Ok(());
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })
    }

    /// Changes the current bio.
    pub fn change_bio(&mut self, bio: Option<UserBio>) -> Result<(), UserError> {
        self.ensure_active_status()?;

        let UserProfile::Ready {
            bio: current_bio, ..
        } = &self.state_required()?.profile
        else {
            return Err(UserError::ProfileNotReady);
        };

        if current_bio.as_ref() == bio.as_ref() {
            return Ok(());
        }

        self.append_event(UserEventPayload::BioChanged { bio })
    }

    /// Links an additional external identity.
    pub fn link_identity(&mut self, identity: UserIdentity) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if let Some(current_identity) =
            self.state_required()?
                .identities
                .iter()
                .find(|current_identity| {
                    current_identity.matches(identity.provider(), identity.subject())
                })
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
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<(), UserError> {
        self.ensure_active_status()?;

        let Some(identity) = self
            .state_required()?
            .identities
            .iter()
            .find(|identity| identity.matches(provider, subject))
        else {
            return Err(UserError::IdentityNotFound);
        };

        if identity.email() == email.as_ref() {
            return Ok(());
        }

        self.append_event(UserEventPayload::IdentityEmailChanged {
            provider: provider.clone(),
            subject: subject.clone(),
            email,
        })
    }

    /// Activates an inactive user.
    pub fn activate(&mut self) -> Result<(), UserError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_active() {
            return Ok(());
        }

        self.append_event(UserEventPayload::Activated)
    }

    /// Deactivates an active user.
    pub fn deactivate(&mut self) -> Result<(), UserError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_inactive() {
            return Ok(());
        }

        self.append_event(UserEventPayload::Inactivated)
    }

    /// Permanently removes a user.
    pub fn remove(&mut self) -> Result<(), UserError> {
        self.ensure_not_removed()?;

        self.append_event(UserEventPayload::Removed)
    }

    fn ensure_not_registered(&self) -> Result<(), UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        Ok(())
    }

    fn ensure_active_status(&self) -> Result<(), UserError> {
        match self.state_required()?.status {
            UserStatus::Active => Ok(()),
            UserStatus::Inactive => Err(UserError::Inactive),
            UserStatus::Removed => Err(UserError::Removed),
        }
    }

    fn ensure_not_removed(&self) -> Result<(), UserError> {
        if self.state_required()?.status.is_removed() {
            return Err(UserError::Removed);
        }

        Ok(())
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered { id, identity } => {
                self.set_state(Some(UserState::new(*id, identity.clone())))
            }
            UserEventPayload::Activated => {
                self.state_required_mut()?.status = UserStatus::Active;
            }
            UserEventPayload::Inactivated => {
                self.state_required_mut()?.status = UserStatus::Inactive;
            }
            UserEventPayload::Removed => {
                self.state_required_mut()?.status = UserStatus::Removed;
            }
            UserEventPayload::ProfileReadied {
                username,
                display_name,
                bio,
            } => {
                self.state_required_mut()?.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    bio: bio.clone(),
                };
            }
            UserEventPayload::UsernameChanged { username } => {
                let state = self.state_required_mut()?;
                let UserProfile::Ready {
                    display_name, bio, ..
                } = &state.profile
                else {
                    return Err(UserError::InvalidProfileState);
                };
                state.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    bio: bio.clone(),
                };
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                let state = self.state_required_mut()?;
                let UserProfile::Ready { username, bio, .. } = &state.profile else {
                    return Err(UserError::InvalidProfileState);
                };
                state.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    bio: bio.clone(),
                };
            }
            UserEventPayload::BioChanged { bio } => {
                let state = self.state_required_mut()?;
                let UserProfile::Ready {
                    username,
                    display_name,
                    ..
                } = &state.profile
                else {
                    return Err(UserError::InvalidProfileState);
                };
                state.profile = UserProfile::Ready {
                    username: username.clone(),
                    display_name: display_name.clone(),
                    bio: bio.clone(),
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
                let identity = state
                    .identities
                    .iter_mut()
                    .find(|identity| identity.matches(provider, subject))
                    .ok_or(UserError::InvalidIdentityState)?;
                identity.change_email(email.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::core::Email;

    use super::{
        User, UserBio, UserDisplayName, UserEventPayload, UserId, UserIdentity,
        UserIdentityProvider, UserIdentitySubject, UserProfile, UserStatus, Username,
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

        assert_eq!(
            user.aggregate_id().expect("aggregate id should exist"),
            user.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(
            user.profile().expect("profile should exist"),
            &UserProfile::Pending
        );
        assert_eq!(
            user.status().expect("status should exist"),
            UserStatus::Active
        );
        assert!(user.is_active().expect("active state should exist"));
        assert!(!user.is_inactive().expect("inactive state should exist"));
        assert!(!user.is_removed().expect("removed state should exist"));
        assert_eq!(
            user.username().expect("username lookup should succeed"),
            None
        );
        assert_eq!(
            user.display_name()
                .expect("display name lookup should succeed"),
            None
        );
        assert_eq!(user.bio().expect("bio lookup should succeed"), None);
        assert_eq!(user.identities().expect("identities should exist").len(), 1);
        assert_eq!(
            user.identities().expect("identities should exist")[0]
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
    fn readying_to_same_profile_returns_error() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let username = Username::try_from("alice").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        let bio = Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        user.ready_profile(username.clone(), display_name.clone(), bio.clone())
            .expect("profile ready should succeed");

        let error = user
            .ready_profile(username, display_name, bio)
            .expect_err("profile ready should fail when already ready");

        assert!(matches!(error, super::UserError::ProfileAlreadyReady));
        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn ready_profile_rejects_already_ready_user_with_different_values() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let bio = Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            bio.clone(),
        )
        .expect("profile ready should succeed");

        let error = user
            .ready_profile(
                Username::try_from("alice_example").expect("username should be valid"),
                UserDisplayName::try_from("Alice Updated").expect("display name should be valid"),
                None,
            )
            .expect_err("readying an already-ready profile should fail");

        assert!(matches!(error, super::UserError::ProfileAlreadyReady));
    }

    #[test]
    fn ready_profile_appends_event_and_updates_state() {
        let username = Username::try_from("alice_example").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        let bio = Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        user.ready_profile(username.clone(), display_name.clone(), bio.clone())
            .expect("profile ready should succeed");

        assert_eq!(
            user.profile().expect("profile should exist"),
            &UserProfile::Ready {
                username: username.clone(),
                display_name: display_name.clone(),
                bio: bio.clone(),
            }
        );
        assert_eq!(
            user.username().expect("username lookup should succeed"),
            Some(&username)
        );
        assert_eq!(
            user.display_name()
                .expect("display name lookup should succeed"),
            Some(&display_name)
        );
        assert_eq!(user.bio().expect("bio lookup should succeed"), bio.as_ref());
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
        let bio = Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            bio.clone(),
        )
        .expect("profile ready should succeed");
        let username = Username::try_from("alice_example").expect("username should be valid");

        user.change_username(username.clone())
            .expect("username change should succeed");

        assert_eq!(
            user.username().expect("username lookup should succeed"),
            Some(&username)
        );
        assert_eq!(user.bio().expect("bio lookup should succeed"), bio.as_ref());
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
        let bio = Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            bio.clone(),
        )
        .expect("profile ready should succeed");
        let display_name =
            UserDisplayName::try_from("Alice Updated").expect("display name should be valid");

        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        assert_eq!(
            user.display_name()
                .expect("display name lookup should succeed"),
            Some(&display_name)
        );
        assert_eq!(user.bio().expect("bio lookup should succeed"), bio.as_ref());
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::DISPLAY_NAME_CHANGED
        );
    }

    #[test]
    fn change_bio_rejects_pending_profile() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        let error = user
            .change_bio(Some(
                UserBio::try_from("Banking enthusiast").expect("bio should be valid"),
            ))
            .expect_err("pending profile should reject bio changes");

        assert!(matches!(error, super::UserError::ProfileNotReady));
    }

    #[test]
    fn change_bio_appends_event_and_updates_state() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        let initial_bio =
            Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid"));
        user.ready_profile(
            Username::try_from("alice").expect("username should be valid"),
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            initial_bio.clone(),
        )
        .expect("profile ready should succeed");
        let updated_bio = Some(UserBio::try_from("Banking operator").expect("bio should be valid"));

        user.change_bio(updated_bio.clone())
            .expect("bio change should succeed");

        assert_eq!(
            user.bio().expect("bio lookup should succeed"),
            updated_bio.as_ref()
        );
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::BIO_CHANGED
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

        assert_eq!(user.identities().expect("identities should exist").len(), 2);
        assert!(
            user.identity(linked_identity.provider(), linked_identity.subject())
                .expect("identity lookup should succeed")
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

        let provider = UserIdentityProvider::try_from("https://login.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-456").expect("subject should be valid");

        let error = user
            .change_identity_email(
                &provider,
                &subject,
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

        user.change_identity_email(&provider, &subject, email.clone())
            .expect("identity email change should succeed");

        assert_eq!(
            user.identity(&provider, &subject)
                .expect("identity lookup should succeed")
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
                bio: Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
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
        let bio_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(6).expect("version should be valid"),
            UserEventPayload::BioChanged { bio: None },
        );
        let identity_email_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(7).expect("version should be valid"),
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
                bio_changed,
                identity_email_changed,
            ],
            None,
        )
        .expect("events should replay");

        assert_eq!(
            user.username()
                .expect("username lookup should succeed")
                .expect("username should exist")
                .value(),
            "alice_updated"
        );
        assert_eq!(
            user.display_name()
                .expect("display name lookup should succeed")
                .expect("display name should exist")
                .value(),
            "Alice Updated"
        );
        assert_eq!(user.bio().expect("bio lookup should succeed"), None);
        assert_eq!(user.identities().expect("identities should exist").len(), 2);
        assert_eq!(
            user.identities().expect("identities should exist")[0]
                .email()
                .expect("email should exist")
                .value(),
            "alice@bank.example"
        );
        assert_eq!(user.version().value(), 7);
        assert!(user.uncommitted_events().is_empty());
    }

    #[test]
    fn replay_rejects_profile_change_before_profile_is_ready() {
        let id = UserId::new();
        let registered = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            UserEventPayload::Registered {
                id,
                identity: identity(),
            },
        );
        let username_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            UserEventPayload::UsernameChanged {
                username: Username::try_from("alice_updated").expect("username should be valid"),
            },
        );
        let mut user = User::default();

        let error = user
            .replay_events(vec![registered, username_changed], None)
            .expect_err("invalid replay should fail");

        assert!(matches!(error, super::UserError::InvalidProfileState));
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

    #[test]
    fn deactivate_and_activate_update_status() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        user.deactivate().expect("deactivate should succeed");
        user.activate().expect("activate should succeed");

        assert_eq!(
            user.status().expect("status should exist"),
            UserStatus::Active
        );
        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::INACTIVATED
        );
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::ACTIVATED
        );
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");

        user.remove().expect("remove should succeed");

        assert_eq!(
            user.status().expect("status should exist"),
            UserStatus::Removed
        );
        assert_eq!(user.uncommitted_events().len(), 2);
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::REMOVED
        );
    }

    #[test]
    fn profile_and_identity_updates_reject_inactive_user() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.deactivate().expect("deactivate should succeed");

        let ready_error = user
            .ready_profile(
                Username::try_from("alice").expect("username should be valid"),
                UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
                None,
            )
            .expect_err("inactive user should reject profile ready");
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let identity_error = user
            .change_identity_email(
                &provider,
                &subject,
                Some(Email::try_from("alice@bank.example").expect("email should be valid")),
            )
            .expect_err("inactive user should reject identity updates");

        assert!(matches!(ready_error, super::UserError::Inactive));
        assert!(matches!(identity_error, super::UserError::Inactive));
    }

    #[test]
    fn operations_reject_removed_user() {
        let mut user = User::default();
        user.register(identity())
            .expect("registration should succeed");
        user.remove().expect("remove should succeed");

        let activate_error = user.activate().expect_err("activate should fail");
        let deactivate_error = user.deactivate().expect_err("deactivate should fail");
        let remove_error = user.remove().expect_err("duplicate remove should fail");
        let ready_error = user
            .ready_profile(
                Username::try_from("alice").expect("username should be valid"),
                UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
                None,
            )
            .expect_err("profile ready should fail");

        assert!(matches!(activate_error, super::UserError::Removed));
        assert!(matches!(deactivate_error, super::UserError::Removed));
        assert!(matches!(remove_error, super::UserError::Removed));
        assert!(matches!(ready_error, super::UserError::Removed));
    }
}
