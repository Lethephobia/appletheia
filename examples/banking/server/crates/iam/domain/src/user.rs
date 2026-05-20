mod user_activate_result;
mod user_bio;
mod user_bio_change_rejection_reason;
mod user_bio_change_result;
mod user_bio_error;
mod user_deactivate_result;
mod user_display_name;
mod user_display_name_change_rejection_reason;
mod user_display_name_change_result;
mod user_display_name_error;
mod user_error;
mod user_event_payload;
mod user_event_payload_error;
mod user_id;
mod user_identity;
mod user_picture_change_rejection_reason;
mod user_picture_change_result;
mod user_picture_object_name;
mod user_picture_object_name_error;
mod user_picture_ref;
mod user_picture_url;
mod user_picture_url_error;
mod user_register_result;
mod user_remove_result;
mod user_state;
mod user_state_error;
mod user_status;
mod user_status_rejection_reason;
mod user_username_change_rejection_reason;
mod user_username_change_result;
mod username;
mod username_error;

pub use user_activate_result::UserActivateResult;
pub use user_bio::UserBio;
pub use user_bio_change_rejection_reason::UserBioChangeRejectionReason;
pub use user_bio_change_result::UserBioChangeResult;
pub use user_bio_error::UserBioError;
pub use user_deactivate_result::UserDeactivateResult;
pub use user_display_name::UserDisplayName;
pub use user_display_name_change_rejection_reason::UserDisplayNameChangeRejectionReason;
pub use user_display_name_change_result::UserDisplayNameChangeResult;
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
pub use user_picture_change_rejection_reason::UserPictureChangeRejectionReason;
pub use user_picture_change_result::UserPictureChangeResult;
pub use user_picture_object_name::UserPictureObjectName;
pub use user_picture_object_name_error::UserPictureObjectNameError;
pub use user_picture_ref::UserPictureRef;
pub use user_picture_url::UserPictureUrl;
pub use user_picture_url_error::UserPictureUrlError;
pub use user_register_result::UserRegisterResult;
pub use user_remove_result::UserRemoveResult;
pub use user_state::UserState;
pub use user_state_error::UserStateError;
pub use user_status::UserStatus;
pub use user_status_rejection_reason::UserStatusRejectionReason;
pub use user_username_change_rejection_reason::UserUsernameChangeRejectionReason;
pub use user_username_change_result::UserUsernameChangeResult;
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
    pub fn register(&mut self) -> Result<UserRegisterResult, UserError> {
        if self.state().is_some() {
            return Err(UserError::AlreadyRegistered);
        }

        let user_id = UserId::new();
        self.append_event(UserEventPayload::Registered { id: user_id })?;
        Ok(UserRegisterResult::Registered { user_id })
    }

    /// Links an additional external identity.
    pub fn link_identity(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<UserIdentityLinkResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserIdentityLinkRejectionReason::Removed;
                self.reject_link_identity(provider, subject, email, reason)?;
                return Ok(UserIdentityLinkResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserIdentityLinkRejectionReason::Inactive;
                self.reject_link_identity(provider, subject, email, reason)?;
                return Ok(UserIdentityLinkResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        if self
            .state_required()?
            .identities
            .iter()
            .any(|current_identity| current_identity.matches(&provider, &subject))
        {
            let reason = UserIdentityLinkRejectionReason::AlreadyLinked;
            self.reject_link_identity(provider, subject, email, reason)?;
            return Ok(UserIdentityLinkResult::Rejected { reason });
        }

        if self.state_required()?.identities.len() >= Self::MAX_IDENTITY_COUNT {
            let reason = UserIdentityLinkRejectionReason::CountLimitExceeded;
            self.reject_link_identity(provider, subject, email, reason)?;
            return Ok(UserIdentityLinkResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::IdentityLinked {
            provider,
            subject,
            email,
        })?;
        Ok(UserIdentityLinkResult::Linked)
    }

    /// Rejects an identity link attempt.
    pub fn reject_link_identity(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        reason: UserIdentityLinkRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::IdentityLinkRejected {
            provider,
            subject,
            email,
            reason,
        })?;
        Ok(())
    }

    /// Changes the email snapshot for a linked identity.
    pub fn change_identity_email(
        &mut self,
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<UserIdentityEmailChangeResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserIdentityEmailChangeRejectionReason::Removed;
                self.reject_change_identity_email(
                    provider.clone(),
                    subject.clone(),
                    email,
                    reason,
                )?;
                return Ok(UserIdentityEmailChangeResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserIdentityEmailChangeRejectionReason::Inactive;
                self.reject_change_identity_email(
                    provider.clone(),
                    subject.clone(),
                    email,
                    reason,
                )?;
                return Ok(UserIdentityEmailChangeResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        let Some(_) = self
            .state_required()?
            .identities
            .iter()
            .find(|identity| identity.matches(provider, subject))
        else {
            let reason = UserIdentityEmailChangeRejectionReason::NotFound;
            self.reject_change_identity_email(provider.clone(), subject.clone(), email, reason)?;
            return Ok(UserIdentityEmailChangeResult::Rejected { reason });
        };

        self.append_event(UserEventPayload::IdentityEmailChanged {
            provider: provider.clone(),
            subject: subject.clone(),
            email,
        })?;
        Ok(UserIdentityEmailChangeResult::Changed)
    }

    /// Rejects an identity email change attempt.
    pub fn reject_change_identity_email(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        reason: UserIdentityEmailChangeRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::IdentityEmailChangeRejected {
            provider,
            subject,
            email,
            reason,
        })?;
        Ok(())
    }

    /// Changes the current username.
    pub fn change_username(
        &mut self,
        username: Username,
    ) -> Result<UserUsernameChangeResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserUsernameChangeRejectionReason::Removed;
                self.reject_change_username(username, reason)?;
                return Ok(UserUsernameChangeResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserUsernameChangeRejectionReason::Inactive;
                self.reject_change_username(username, reason)?;
                return Ok(UserUsernameChangeResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        self.append_event(UserEventPayload::UsernameChanged { username })?;
        Ok(UserUsernameChangeResult::Changed)
    }

    /// Rejects a username change attempt.
    pub fn reject_change_username(
        &mut self,
        username: Username,
        reason: UserUsernameChangeRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::UsernameChangeRejected { username, reason })?;
        Ok(())
    }

    /// Changes the current display name.
    pub fn change_display_name(
        &mut self,
        display_name: UserDisplayName,
    ) -> Result<UserDisplayNameChangeResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserDisplayNameChangeRejectionReason::Removed;
                self.reject_change_display_name(display_name, reason)?;
                return Ok(UserDisplayNameChangeResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserDisplayNameChangeRejectionReason::Inactive;
                self.reject_change_display_name(display_name, reason)?;
                return Ok(UserDisplayNameChangeResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        self.append_event(UserEventPayload::DisplayNameChanged { display_name })?;
        Ok(UserDisplayNameChangeResult::Changed)
    }

    /// Rejects a display name change attempt.
    pub fn reject_change_display_name(
        &mut self,
        display_name: UserDisplayName,
        reason: UserDisplayNameChangeRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::DisplayNameChangeRejected {
            display_name,
            reason,
        })?;
        Ok(())
    }

    /// Changes the current bio.
    pub fn change_bio(&mut self, bio: Option<UserBio>) -> Result<UserBioChangeResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserBioChangeRejectionReason::Removed;
                self.reject_change_bio(bio, reason)?;
                return Ok(UserBioChangeResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserBioChangeRejectionReason::Inactive;
                self.reject_change_bio(bio, reason)?;
                return Ok(UserBioChangeResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        self.append_event(UserEventPayload::BioChanged { bio })?;
        Ok(UserBioChangeResult::Changed)
    }

    /// Rejects a bio change attempt.
    pub fn reject_change_bio(
        &mut self,
        bio: Option<UserBio>,
        reason: UserBioChangeRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::BioChangeRejected { bio, reason })?;
        Ok(())
    }

    /// Changes the current picture.
    pub fn change_picture(
        &mut self,
        picture: Option<UserPictureRef>,
    ) -> Result<UserPictureChangeResult, UserError> {
        match self.state_required()?.status {
            UserStatus::Removed => {
                let reason = UserPictureChangeRejectionReason::Removed;
                self.reject_change_picture(picture, reason)?;
                return Ok(UserPictureChangeResult::Rejected { reason });
            }
            UserStatus::Inactive => {
                let reason = UserPictureChangeRejectionReason::Inactive;
                self.reject_change_picture(picture, reason)?;
                return Ok(UserPictureChangeResult::Rejected { reason });
            }
            UserStatus::Active => {}
        }

        let old_picture = self.state_required()?.picture.clone();

        self.append_event(UserEventPayload::PictureChanged {
            picture,
            old_picture,
        })?;
        Ok(UserPictureChangeResult::Changed)
    }

    /// Rejects a picture change attempt.
    pub fn reject_change_picture(
        &mut self,
        picture: Option<UserPictureRef>,
        reason: UserPictureChangeRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::PictureChangeRejected { picture, reason })?;
        Ok(())
    }

    /// Activates an inactive user.
    pub fn activate(&mut self) -> Result<UserActivateResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.reject_activate(reason)?;
            return Ok(UserActivateResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::Activated)?;
        Ok(UserActivateResult::Activated)
    }

    /// Rejects a user activation attempt.
    pub fn reject_activate(&mut self, reason: UserStatusRejectionReason) -> Result<(), UserError> {
        self.append_event(UserEventPayload::ActivateRejected { reason })?;
        Ok(())
    }

    /// Deactivates an active user.
    pub fn deactivate(&mut self) -> Result<UserDeactivateResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.reject_deactivate(reason)?;
            return Ok(UserDeactivateResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::Deactivated)?;
        Ok(UserDeactivateResult::Deactivated)
    }

    /// Rejects a user deactivation attempt.
    pub fn reject_deactivate(
        &mut self,
        reason: UserStatusRejectionReason,
    ) -> Result<(), UserError> {
        self.append_event(UserEventPayload::DeactivateRejected { reason })?;
        Ok(())
    }

    /// Permanently removes a user.
    pub fn remove(&mut self) -> Result<UserRemoveResult, UserError> {
        if self.state_required()?.status.is_removed() {
            let reason = UserStatusRejectionReason::Removed;
            self.reject_remove(reason)?;
            return Ok(UserRemoveResult::Rejected { reason });
        }

        self.append_event(UserEventPayload::Removed)?;
        Ok(UserRemoveResult::Removed)
    }

    /// Rejects a user removal attempt.
    pub fn reject_remove(&mut self, reason: UserStatusRejectionReason) -> Result<(), UserError> {
        self.append_event(UserEventPayload::RemoveRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<UserEventPayload, UserError> for User {
    fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
        match payload {
            UserEventPayload::Registered { id } => self.set_state(Some(UserState {
                id: *id,
                identities: Vec::new(),
                username: None,
                display_name: None,
                bio: None,
                picture: None,
                status: UserStatus::Active,
            })),
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
            UserEventPayload::Deactivated => {
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
        Email, User, UserBio, UserDisplayName, UserDisplayNameChangeRejectionReason,
        UserDisplayNameChangeResult, UserEventPayload, UserIdentityEmailChangeRejectionReason,
        UserIdentityEmailChangeResult, UserIdentityLinkRejectionReason, UserIdentityLinkResult,
        UserIdentityProvider, UserIdentitySubject, UserPictureRef, UserPictureUrl, UserStatus,
        UserUsernameChangeRejectionReason, UserUsernameChangeResult, Username,
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
    fn identical_username_change_appends_success_event() {
        let mut user = User::default();
        let username = Username::try_from("alice").expect("username should be valid");
        register_user(&mut user);
        user.change_username(username.clone())
            .expect("username change should succeed");

        user.change_username(username)
            .expect("idempotent change should succeed");

        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::USERNAME_CHANGED
        );
    }

    #[test]
    fn identical_display_name_change_appends_success_event() {
        let mut user = User::default();
        let display_name = display_name();
        register_user(&mut user);
        user.change_display_name(display_name.clone())
            .expect("display name change should succeed");

        user.change_display_name(display_name)
            .expect("idempotent display name change should succeed");

        assert_eq!(user.uncommitted_events().len(), 3);
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::DISPLAY_NAME_CHANGED
        );
    }

    #[test]
    fn identical_identity_email_change_appends_success_event() {
        let mut user = User::default();
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let email = Some(Email::try_from("alice@example.com").expect("email should be valid"));
        register_user(&mut user);
        user.link_identity(provider.clone(), subject.clone(), email.clone())
            .expect("identity should link");

        user.change_identity_email(&provider, &subject, email)
            .expect("idempotent identity email change should succeed");

        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::IDENTITY_EMAIL_CHANGED
        );
    }

    #[test]
    fn repeated_activation_and_deactivation_append_success_events() {
        let mut user = User::default();
        register_user(&mut user);

        user.activate().expect("activation should succeed");
        user.deactivate().expect("deactivation should succeed");
        user.deactivate()
            .expect("repeated deactivation should succeed");

        assert_eq!(
            user.uncommitted_events()[1].payload().name(),
            UserEventPayload::ACTIVATED
        );
        assert_eq!(
            user.uncommitted_events()[2].payload().name(),
            UserEventPayload::DEACTIVATED
        );
        assert_eq!(
            user.uncommitted_events()[3].payload().name(),
            UserEventPayload::DEACTIVATED
        );
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
            UserUsernameChangeResult::Rejected {
                reason: UserUsernameChangeRejectionReason::Inactive
            }
        ));
        assert!(matches!(
            display_name_result,
            UserDisplayNameChangeResult::Rejected {
                reason: UserDisplayNameChangeRejectionReason::Inactive
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
