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
pub use user_picture_object_name::UserPictureObjectName;
pub use user_picture_object_name_error::UserPictureObjectNameError;
pub use user_picture_ref::UserPictureRef;
pub use user_picture_url::UserPictureUrl;
pub use user_picture_url_error::UserPictureUrlError;
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
    pub fn profile(&self) -> Result<Option<&UserProfile>, UserError> {
        Ok(self.state_required()?.profile.as_ref())
    }

    /// Returns the current username.
    pub fn username(&self) -> Result<Option<&Username>, UserError> {
        Ok(self.state_required()?.username.as_ref())
    }

    /// Returns the current display name.
    pub fn display_name(&self) -> Result<Option<&UserDisplayName>, UserError> {
        Ok(self
            .state_required()?
            .profile
            .as_ref()
            .map(UserProfile::display_name))
    }

    /// Returns the current bio.
    pub fn bio(&self) -> Result<Option<&UserBio>, UserError> {
        Ok(self
            .state_required()?
            .profile
            .as_ref()
            .and_then(UserProfile::bio))
    }

    /// Returns the current picture.
    pub fn picture(&self) -> Result<Option<&UserPictureRef>, UserError> {
        Ok(self
            .state_required()?
            .profile
            .as_ref()
            .and_then(UserProfile::picture))
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
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        self.append_event(UserEventPayload::Registered {
            id: UserId::new(),
            identity,
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

    /// Changes the current profile.
    pub fn change_profile(&mut self, profile: UserProfile) -> Result<(), UserError> {
        self.ensure_active_status()?;

        if self.state_required()?.profile.as_ref() == Some(&profile) {
            return Ok(());
        }

        self.append_event(UserEventPayload::ProfileChanged { profile })
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
            UserEventPayload::Activated => {
                self.state_required_mut()?.status = UserStatus::Active;
            }
            UserEventPayload::Inactivated => {
                self.state_required_mut()?.status = UserStatus::Inactive;
            }
            UserEventPayload::Removed => {
                self.state_required_mut()?.status = UserStatus::Removed;
            }
            UserEventPayload::UsernameChanged { username } => {
                self.state_required_mut()?.username = Some(username.clone());
            }
            UserEventPayload::ProfileChanged { profile } => {
                self.state_required_mut()?.profile = Some(profile.clone());
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use super::{
        User, UserBio, UserDisplayName, UserError, UserEventPayload, UserIdentity,
        UserIdentityProvider, UserIdentitySubject, UserPictureRef, UserPictureUrl, UserProfile,
        UserStatus, Username,
    };

    fn identity() -> UserIdentity {
        UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        )
    }

    fn profile() -> UserProfile {
        UserProfile::new(
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
            Some(UserPictureRef::external_url(
                UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                    .expect("picture URL should be valid"),
            )),
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
        assert_eq!(user.profile().expect("profile should exist"), None);
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
    fn change_profile_sets_profile() {
        let mut user = User::default();
        let profile = profile();
        user.register(identity()).expect("user should register");

        user.change_profile(profile.clone())
            .expect("profile change should succeed");

        assert_eq!(
            user.profile().expect("profile should exist"),
            Some(&profile)
        );
        assert_eq!(
            user.display_name().expect("display name should exist"),
            Some(profile.display_name())
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
    fn identical_profile_change_is_a_no_op() {
        let mut user = User::default();
        let profile = profile();
        user.register(identity()).expect("user should register");
        user.change_profile(profile.clone())
            .expect("profile change should succeed");

        user.change_profile(profile)
            .expect("idempotent profile change should succeed");

        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn profile_and_username_changes_reject_inactive_user() {
        let mut user = User::default();
        user.register(identity()).expect("user should register");
        user.deactivate().expect("user should deactivate");

        let username_error = user
            .change_username(Username::try_from("alice").expect("username should be valid"))
            .expect_err("inactive user should reject username changes");
        let profile_error = user
            .change_profile(profile())
            .expect_err("inactive user should reject profile changes");

        assert!(matches!(username_error, UserError::Inactive));
        assert!(matches!(profile_error, UserError::Inactive));
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
