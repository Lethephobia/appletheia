mod user_activate_result;
mod user_bio;
mod user_bio_error;
mod user_deactivate_result;
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
mod user_profile_change_rejection_reason;
mod user_profile_change_result;
mod user_remove_result;
mod user_state;
mod user_state_error;
mod user_status;
mod user_status_rejection_reason;
mod username;
mod username_error;

pub use user_activate_result::UserActivateResult;
pub use user_bio::UserBio;
pub use user_bio_error::UserBioError;
pub use user_deactivate_result::UserDeactivateResult;
pub use user_display_name::UserDisplayName;
pub use user_display_name_error::UserDisplayNameError;
pub use user_error::UserError;
pub use user_event_payload::UserEventPayload;
pub use user_event_payload_error::UserEventPayloadError;
pub use user_id::UserId;
pub use user_identity::{
    UserIdentity, UserIdentityEmailChangeRejectionReason, UserIdentityEmailChangeResult,
    UserIdentityLinkRejectionReason, UserIdentityLinkResult, UserIdentityProvider,
    UserIdentityProviderError, UserIdentitySubject, UserIdentitySubjectError,
};
pub use user_picture_object_name::UserPictureObjectName;
pub use user_picture_object_name_error::UserPictureObjectNameError;
pub use user_picture_ref::UserPictureRef;
pub use user_picture_url::UserPictureUrl;
pub use user_picture_url_error::UserPictureUrlError;
pub use user_profile_change_rejection_reason::UserProfileChangeRejectionReason;
pub use user_profile_change_result::UserProfileChangeResult;
pub use user_remove_result::UserRemoveResult;
pub use user_state::UserState;
pub use user_state_error::UserStateError;
pub use user_status::UserStatus;
pub use user_status_rejection_reason::UserStatusRejectionReason;
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
    pub const MAX_IDENTITY_COUNT: usize = 32;

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

    /// Registers a new user.
    pub fn register(&mut self) -> Result<(), UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        self.append_event(UserEventPayload::Registered { id: UserId::new() })
    }

    /// Links an additional external identity.
    pub fn link_identity(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<UserIdentityLinkResult, UserError> {
        if let Some(reason) = self.identity_link_rejection_reason()? {
            self.append_event(UserEventPayload::IdentityLinkRejected {
                provider,
                subject,
                email,
                reason,
            })?;
            return Ok(UserIdentityLinkResult::Rejected { reason });
        }

        if self
            .state_required()?
            .identities
            .iter()
            .any(|current_identity| current_identity.matches(&provider, &subject))
        {
            let reason = UserIdentityLinkRejectionReason::AlreadyLinked;
            self.append_event(UserEventPayload::IdentityLinkRejected {
                provider,
                subject,
                email,
                reason,
            })?;
            return Ok(UserIdentityLinkResult::Rejected { reason });
        }

        if self.state_required()?.identities.len() >= Self::MAX_IDENTITY_COUNT {
            let reason = UserIdentityLinkRejectionReason::CountLimitExceeded;
            self.append_event(UserEventPayload::IdentityLinkRejected {
                provider,
                subject,
                email,
                reason,
            })?;
            return Ok(UserIdentityLinkResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::IdentityLinked {
            provider,
            subject,
            email,
        })?;
        Ok(UserIdentityLinkResult::Linked)
    }

    /// Changes the email snapshot for a linked identity.
    pub fn change_identity_email(
        &mut self,
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<UserIdentityEmailChangeResult, UserError> {
        if let Some(reason) = self.identity_email_change_rejection_reason()? {
            self.append_event(UserEventPayload::IdentityEmailChangeRejected {
                provider: provider.clone(),
                subject: subject.clone(),
                email,
                reason,
            })?;
            return Ok(UserIdentityEmailChangeResult::Rejected { reason });
        }

        let Some(identity) = self
            .state_required()?
            .identities
            .iter()
            .find(|identity| identity.matches(provider, subject))
        else {
            let reason = UserIdentityEmailChangeRejectionReason::NotFound;
            self.append_event(UserEventPayload::IdentityEmailChangeRejected {
                provider: provider.clone(),
                subject: subject.clone(),
                email,
                reason,
            })?;
            return Ok(UserIdentityEmailChangeResult::Rejected { reason });
        };

        if identity.email() == email.as_ref() {
            return Ok(UserIdentityEmailChangeResult::Changed);
        }

        self.append_event(UserEventPayload::IdentityEmailChanged {
            provider: provider.clone(),
            subject: subject.clone(),
            email,
        })?;
        Ok(UserIdentityEmailChangeResult::Changed)
    }

    /// Changes the current username.
    pub fn change_username(
        &mut self,
        username: Username,
    ) -> Result<UserProfileChangeResult, UserError> {
        if let Some(reason) = self.profile_change_rejection_reason()? {
            self.append_event(UserEventPayload::UsernameChangeRejected { username, reason })?;
            return Ok(UserProfileChangeResult::Rejected { reason });
        }

        if self.state_required()?.username.as_ref() == Some(&username) {
            return Ok(UserProfileChangeResult::Changed);
        }

        self.append_event(UserEventPayload::UsernameChanged { username })?;
        Ok(UserProfileChangeResult::Changed)
    }

    /// Changes the current display name.
    pub fn change_display_name(
        &mut self,
        display_name: UserDisplayName,
    ) -> Result<UserProfileChangeResult, UserError> {
        if let Some(reason) = self.profile_change_rejection_reason()? {
            self.append_event(UserEventPayload::DisplayNameChangeRejected {
                display_name,
                reason,
            })?;
            return Ok(UserProfileChangeResult::Rejected { reason });
        }

        if self.state_required()?.display_name.as_ref() == Some(&display_name) {
            return Ok(UserProfileChangeResult::Changed);
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })?;
        Ok(UserProfileChangeResult::Changed)
    }

    /// Changes the current bio.
    pub fn change_bio(
        &mut self,
        bio: Option<UserBio>,
    ) -> Result<UserProfileChangeResult, UserError> {
        if let Some(reason) = self.profile_change_rejection_reason()? {
            self.append_event(UserEventPayload::BioChangeRejected { bio, reason })?;
            return Ok(UserProfileChangeResult::Rejected { reason });
        }

        if self.state_required()?.bio == bio {
            return Ok(UserProfileChangeResult::Changed);
        }

        self.append_event(UserEventPayload::BioChanged { bio })?;
        Ok(UserProfileChangeResult::Changed)
    }

    /// Changes the current picture.
    pub fn change_picture(
        &mut self,
        picture: Option<UserPictureRef>,
    ) -> Result<UserProfileChangeResult, UserError> {
        if let Some(reason) = self.profile_change_rejection_reason()? {
            self.append_event(UserEventPayload::PictureChangeRejected { picture, reason })?;
            return Ok(UserProfileChangeResult::Rejected { reason });
        }

        if self.state_required()?.picture == picture {
            return Ok(UserProfileChangeResult::Changed);
        }

        let old_picture = self.state_required()?.picture.clone();

        self.append_event(UserEventPayload::PictureChanged {
            picture,
            old_picture,
        })?;
        Ok(UserProfileChangeResult::Changed)
    }

    /// Activates an inactive user.
    pub fn activate(&mut self) -> Result<UserActivateResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.append_event(UserEventPayload::ActivateRejected { reason })?;
            return Ok(UserActivateResult::Rejected { reason });
        }

        if self.state_required()?.status.is_active() {
            return Ok(UserActivateResult::Activated);
        }

        self.append_event(UserEventPayload::Activated)?;
        Ok(UserActivateResult::Activated)
    }

    /// Deactivates an active user.
    pub fn deactivate(&mut self) -> Result<UserDeactivateResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.append_event(UserEventPayload::DeactivateRejected { reason })?;
            return Ok(UserDeactivateResult::Rejected { reason });
        }

        if self.state_required()?.status.is_inactive() {
            return Ok(UserDeactivateResult::Deactivated);
        }

        self.append_event(UserEventPayload::Inactivated)?;
        Ok(UserDeactivateResult::Deactivated)
    }

    /// Permanently removes a user.
    pub fn remove(&mut self) -> Result<UserRemoveResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.append_event(UserEventPayload::RemoveRejected { reason })?;
            return Ok(UserRemoveResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::Removed)?;
        Ok(UserRemoveResult::Removed)
    }

    fn identity_link_rejection_reason(
        &self,
    ) -> Result<Option<UserIdentityLinkRejectionReason>, UserError> {
        if self.state_required()?.status.is_removed() {
            return Ok(Some(UserIdentityLinkRejectionReason::Removed));
        }

        if self.state_required()?.status.is_inactive() {
            return Ok(Some(UserIdentityLinkRejectionReason::Inactive));
        }

        Ok(None)
    }

    fn identity_email_change_rejection_reason(
        &self,
    ) -> Result<Option<UserIdentityEmailChangeRejectionReason>, UserError> {
        if self.state_required()?.status.is_removed() {
            return Ok(Some(UserIdentityEmailChangeRejectionReason::Removed));
        }

        if self.state_required()?.status.is_inactive() {
            return Ok(Some(UserIdentityEmailChangeRejectionReason::Inactive));
        }

        Ok(None)
    }

    fn profile_change_rejection_reason(
        &self,
    ) -> Result<Option<UserProfileChangeRejectionReason>, UserError> {
        if self.state_required()?.status.is_removed() {
            return Ok(Some(UserProfileChangeRejectionReason::Removed));
        }

        if self.state_required()?.status.is_inactive() {
            return Ok(Some(UserProfileChangeRejectionReason::Inactive));
        }

        Ok(None)
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered { id } => self.set_state(Some(UserState::new(*id))),
            UserEventPayload::IdentityLinked {
                provider,
                subject,
                email,
            } => {
                self.state_required_mut()?
                    .identities
                    .push(UserIdentity::new(
                        provider.clone(),
                        subject.clone(),
                        email.clone(),
                    ));
            }
            UserEventPayload::IdentityLinkRejected { .. } => {}
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
            UserEventPayload::IdentityEmailChangeRejected { .. } => {}
            UserEventPayload::UsernameChanged { username } => {
                self.state_required_mut()?.username = Some(username.clone());
            }
            UserEventPayload::UsernameChangeRejected { .. } => {}
            UserEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = Some(display_name.clone());
            }
            UserEventPayload::DisplayNameChangeRejected { .. } => {}
            UserEventPayload::BioChanged { bio } => {
                self.state_required_mut()?.bio = bio.clone();
            }
            UserEventPayload::BioChangeRejected { .. } => {}
            UserEventPayload::PictureChanged { picture, .. } => {
                self.state_required_mut()?.picture = picture.clone();
            }
            UserEventPayload::PictureChangeRejected { .. } => {}
            UserEventPayload::Activated => {
                self.state_required_mut()?.status = UserStatus::Active;
            }
            UserEventPayload::ActivateRejected { .. } => {}
            UserEventPayload::Inactivated => {
                self.state_required_mut()?.status = UserStatus::Inactive;
            }
            UserEventPayload::DeactivateRejected { .. } => {}
            UserEventPayload::Removed => {
                self.state_required_mut()?.status = UserStatus::Removed;
            }
            UserEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use super::{
        User, UserBio, UserDisplayName, UserEventPayload, UserIdentityEmailChangeRejectionReason,
        UserIdentityEmailChangeResult, UserIdentityLinkRejectionReason, UserIdentityLinkResult,
        UserIdentityProvider, UserIdentitySubject, UserPictureRef, UserPictureUrl,
        UserProfileChangeRejectionReason, UserProfileChangeResult, UserStatus, Username,
    };

    fn register_user(user: &mut User) {
        user.register().expect("user should register");
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

        register_user(&mut user);

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
        register_user(&mut user);

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
        register_user(&mut user);

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
        register_user(&mut user);
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
        register_user(&mut user);
        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        user.change_display_name(display_name)
            .expect("idempotent display name change should succeed");

        assert_eq!(user.uncommitted_events().len(), 2);
    }

    #[test]
    fn bio_and_picture_changes_update_state() {
        let mut user = User::default();
        register_user(&mut user);

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
        register_user(&mut user);
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
        register_user(&mut user);
        user.deactivate().expect("user should deactivate");

        let username_result = user
            .change_username(Username::try_from("alice").expect("username should be valid"))
            .expect("inactive user rejection should be recorded");
        let display_name_result = user
            .change_display_name(display_name())
            .expect("inactive user rejection should be recorded");

        assert!(matches!(
            username_result,
            UserProfileChangeResult::Rejected {
                reason: UserProfileChangeRejectionReason::Inactive
            }
        ));
        assert!(matches!(
            display_name_result,
            UserProfileChangeResult::Rejected {
                reason: UserProfileChangeRejectionReason::Inactive
            }
        ));
    }

    #[test]
    fn identity_email_change_rejects_unknown_identity() {
        let mut user = User::default();
        register_user(&mut user);

        let result = user
            .change_identity_email(
                &UserIdentityProvider::try_from("https://other.example.com")
                    .expect("provider should be valid"),
                &UserIdentitySubject::try_from("user-999").expect("subject should be valid"),
                None,
            )
            .expect("unknown identity rejection should be recorded");

        assert!(matches!(
            result,
            UserIdentityEmailChangeResult::Rejected {
                reason: UserIdentityEmailChangeRejectionReason::NotFound
            }
        ));
    }

    #[test]
    fn link_identity_rejects_identity_count_over_limit() {
        let mut user = User::default();
        register_user(&mut user);

        for index in 0..User::MAX_IDENTITY_COUNT {
            user.link_identity(
                UserIdentityProvider::try_from(format!("https://accounts-{index}.example.com"))
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from(format!("user-{index}"))
                    .expect("subject should be valid"),
                None,
            )
            .expect("identity should link");
        }

        let result = user
            .link_identity(
                UserIdentityProvider::try_from("https://accounts-over-limit.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-over-limit").expect("subject should be valid"),
                None,
            )
            .expect("identity count over limit rejection should be recorded");

        assert!(matches!(
            result,
            UserIdentityLinkResult::Rejected {
                reason: UserIdentityLinkRejectionReason::CountLimitExceeded
            }
        ));
    }

    #[test]
    fn link_identity_rejects_already_linked_identity() {
        let mut user = User::default();
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        register_user(&mut user);
        user.link_identity(provider.clone(), subject.clone(), None)
            .expect("identity should link");

        let result = user
            .link_identity(provider, subject, None)
            .expect("duplicate identity rejection should be recorded");

        assert!(matches!(
            result,
            UserIdentityLinkResult::Rejected {
                reason: UserIdentityLinkRejectionReason::AlreadyLinked
            }
        ));
        assert_eq!(user.identities().expect("identities should exist").len(), 1);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::IDENTITY_LINK_REJECTED
        );
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let mut user = User::default();
        register_user(&mut user);

        user.remove().expect("remove should succeed");

        assert!(user.is_removed().expect("removed status should exist"));
        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::REMOVED
        );
    }
}
