mod user_bio;
mod user_bio_error;
mod user_display_name;
mod user_display_name_error;
mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_identity;
mod user_picture_object_name;
mod user_picture_object_name_error;
mod user_picture_ref;
mod user_picture_url;
mod user_picture_url_error;
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
pub use user_picture_object_name::UserPictureObjectName;
pub use user_picture_object_name_error::UserPictureObjectNameError;
pub use user_picture_ref::UserPictureRef;
pub use user_picture_url::UserPictureUrl;
pub use user_picture_url_error::UserPictureUrlError;
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

    /// Returns the current username.
    pub fn username(&self) -> Result<Option<&Username>, UserError> {
        Ok(self.state_required()?.username.as_ref())
    }

    /// Returns the current display name.
    pub fn display_name(&self) -> Result<Option<&UserDisplayName>, UserError> {
        Ok(self.state_required()?.display_name.as_ref())
    }

    /// Returns the current bio.
    pub fn bio(&self) -> Result<Option<&UserBio>, UserError> {
        Ok(self.state_required()?.bio.as_ref())
    }

    /// Returns the current picture.
    pub fn picture(&self) -> Result<Option<&UserPictureRef>, UserError> {
        Ok(self.state_required()?.picture.as_ref())
    }

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
            if current_identity == &identity {
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

    /// Changes the current username.
    pub fn change_username(&mut self, username: Username) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if self.state_required()?.username.as_ref() == Some(&username) {
            return Ok(());
        }

        self.append_event(UserEventPayload::UsernameChanged { username })
    }

    /// Changes the current display name.
    pub fn change_display_name(&mut self, display_name: UserDisplayName) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if self.state_required()?.display_name.as_ref() == Some(&display_name) {
            return Ok(());
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })
    }

    /// Changes the current bio.
    pub fn change_bio(&mut self, bio: Option<UserBio>) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if self.state_required()?.bio == bio {
            return Ok(());
        }

        self.append_event(UserEventPayload::BioChanged { bio })
    }

    /// Changes the current picture.
    pub fn change_picture(&mut self, picture: Option<UserPictureRef>) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if self.state_required()?.picture == picture {
            return Ok(());
        }

        let old_picture = self.state_required()?.picture.clone();

        self.append_event(UserEventPayload::PictureChanged {
            picture,
            old_picture,
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

    fn ensure_active_status(&self) -> Result<(), UserError> {
        if self.state_required()?.status.is_removed() {
            return Err(UserError::Removed);
        }

        if self.state_required()?.status.is_inactive() {
            return Err(UserError::Inactive);
        }

        Ok(())
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
            UserEventPayload::IdentityLinked { identity } => {
                self.state_required_mut()?.identities.push(identity.clone());
            }
            UserEventPayload::IdentityEmailChanged {
                provider,
                subject,
                email,
            } => {
                let identity = self
                    .state_required_mut()?
                    .identities
                    .iter_mut()
                    .find(|identity| identity.matches(provider, subject))
                    .ok_or(UserError::InvalidIdentityState)?;
                identity.change_email(email.clone());
            }
            UserEventPayload::UsernameChanged { username } => {
                self.state_required_mut()?.username = Some(username.clone());
            }
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = Some(display_name.clone());
            }
            UserEventPayload::BioChanged { bio } => {
                self.state_required_mut()?.bio = bio.clone();
            }
            UserEventPayload::PictureChanged { picture, .. } => {
                self.state_required_mut()?.picture = picture.clone();
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use super::{
        User, UserBio, UserDisplayName, UserError, UserEventPayload, UserIdentity,
        UserIdentityProvider, UserIdentitySubject, UserPictureRef, UserPictureUrl, UserStatus,
        Username,
    };

    fn identity() -> UserIdentity {
        UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        )
    }

    fn display_name() -> UserDisplayName {
        UserDisplayName::try_from("Alice Example").expect("display name should be valid")
    }

    fn bio() -> UserBio {
        UserBio::try_from("Banking enthusiast").expect("bio should be valid")
    }

    fn picture() -> UserPictureRef {
        UserPictureRef::external_url(
            UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                .expect("picture URL should be valid"),
        )
    }

    #[test]
    fn register_initializes_state_and_records_event() {
        let mut user = User::default();

        user.register(identity()).expect("user should register");

        assert_eq!(
            user.status().expect("status should exist"),
            UserStatus::Active
        );
        assert_eq!(user.username().expect("username should exist"), None);
        assert_eq!(
            user.display_name().expect("display name should exist"),
            None
        );
        assert_eq!(user.bio().expect("bio should exist"), None);
        assert_eq!(user.picture().expect("picture should exist"), None);
        assert_eq!(
            user.uncommitted_events()[0].payload().name(),
            UserEventPayload::REGISTERED
        );
    }

    #[test]
    fn change_username_sets_username() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");

        user.change_username(Username::try_from("alice").expect("username should be valid"))
            .expect("username change should succeed");

        assert_eq!(
            user.username().expect("username should exist"),
            Some(&Username::try_from("alice").expect("username should be valid"))
        );
    }

    #[test]
    fn change_display_name_sets_display_name() {
        let mut user = User::default();
        let display_name = display_name();
        user.register(identity()).expect("user should register");

        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        assert_eq!(
            user.display_name().expect("display name should exist"),
            Some(&display_name)
        );
    }

    #[test]
    fn identical_username_change_is_a_no_op() {
        let mut user = User::default();
        let username = Username::try_from("alice").expect("username should be valid");
        user.register(identity()).expect("user should register");
        user.change_username(username.clone())
            .expect("username change should succeed");

        user.change_username(username)
            .expect("idempotent change should succeed");

        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn identical_display_name_change_is_a_no_op() {
        let mut user = User::default();
        let display_name = display_name();
        user.register(identity()).expect("user should register");
        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        user.change_display_name(display_name)
            .expect("idempotent display name change should succeed");

        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn bio_and_picture_changes_update_state() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");

        user.change_bio(Some(bio()))
            .expect("bio change should succeed");
        user.change_picture(Some(picture()))
            .expect("picture change should succeed");

        assert_eq!(
            user.bio().expect("bio should exist").map(UserBio::value),
            Some("Banking enthusiast")
        );
        assert!(user.picture().expect("picture should exist").is_some());
    }

    #[test]
    fn picture_changed_event_records_old_picture_after_current_picture() {
        let mut user = User::default();
        let first_picture = picture();
        let second_picture = UserPictureRef::external_url(
            UserPictureUrl::try_from("https://cdn.example.com/alice-updated.png")
                .expect("picture URL should be valid"),
        );
        user.register(identity()).expect("user should register");
        user.change_picture(Some(first_picture.clone()))
            .expect("picture change should succeed");

        user.change_picture(Some(second_picture.clone()))
            .expect("picture change should succeed");

        let UserEventPayload::PictureChanged {
            picture,
            old_picture,
        } = user.uncommitted_events()[2].payload()
        else {
            panic!("event should be picture changed");
        };
        assert_eq!(picture.as_ref(), Some(&second_picture));
        assert_eq!(old_picture.as_ref(), Some(&first_picture));
    }

    #[test]
    fn display_name_and_username_changes_reject_inactive_user() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");
        user.deactivate().expect("user should deactivate");

        let username_error = user
            .change_username(Username::try_from("alice").expect("username should be valid"))
            .expect_err("inactive user should reject username changes");
        let display_name_error = user
            .change_display_name(display_name())
            .expect_err("inactive user should reject display name changes");

        assert!(matches!(username_error, UserError::Inactive));
        assert!(matches!(display_name_error, UserError::Inactive));
    }

    #[test]
    fn identity_email_change_rejects_unknown_identity() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");

        let error = user
            .change_identity_email(
                &UserIdentityProvider::try_from("https://other.example.com")
                    .expect("provider should be valid"),
                &UserIdentitySubject::try_from("user-999").expect("subject should be valid"),
                None,
            )
            .expect_err("unknown identity should be rejected");

        assert!(matches!(error, UserError::IdentityNotFound));
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");

        user.remove().expect("remove should succeed");

        assert!(user.is_removed().expect("removed status should exist"));
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::REMOVED
        );
    }
}
