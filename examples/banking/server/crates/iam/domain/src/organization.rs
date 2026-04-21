mod organization_description;
mod organization_description_error;
mod organization_display_name;
mod organization_display_name_error;
mod organization_error;
mod organization_event_payload;
mod organization_event_payload_error;
mod organization_handle;
mod organization_handle_error;
mod organization_id;
mod organization_owner;
mod organization_picture_object_name;
mod organization_picture_object_name_error;
mod organization_picture_ref;
mod organization_picture_url;
mod organization_picture_url_error;
mod organization_profile;
mod organization_state;
mod organization_state_error;
mod organization_status;
mod organization_website_url;
mod organization_website_url_error;

pub use organization_description::OrganizationDescription;
pub use organization_description_error::OrganizationDescriptionError;
pub use organization_display_name::OrganizationDisplayName;
pub use organization_display_name_error::OrganizationDisplayNameError;
pub use organization_error::OrganizationError;
pub use organization_event_payload::OrganizationEventPayload;
pub use organization_event_payload_error::OrganizationEventPayloadError;
pub use organization_handle::OrganizationHandle;
pub use organization_handle_error::OrganizationHandleError;
pub use organization_id::OrganizationId;
pub use organization_owner::OrganizationOwner;
pub use organization_picture_object_name::OrganizationPictureObjectName;
pub use organization_picture_object_name_error::OrganizationPictureObjectNameError;
pub use organization_picture_ref::OrganizationPictureRef;
pub use organization_picture_url::OrganizationPictureUrl;
pub use organization_picture_url_error::OrganizationPictureUrlError;
pub use organization_profile::OrganizationProfile;
pub use organization_state::OrganizationState;
pub use organization_state_error::OrganizationStateError;
pub use organization_status::OrganizationStatus;
pub use organization_website_url::OrganizationWebsiteUrl;
pub use organization_website_url_error::OrganizationWebsiteUrlError;

pub type OrganizationName = OrganizationDisplayName;
pub type OrganizationNameError = OrganizationDisplayNameError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `Organization` aggregate root.
#[aggregate(type = "organization", error = OrganizationError)]
pub struct Organization {
    core: AggregateCore<OrganizationState, OrganizationEventPayload>,
}

impl Organization {
    /// Returns the current organization profile.
    pub fn profile(&self) -> Result<&OrganizationProfile, OrganizationError> {
        Ok(&self.state_required()?.profile)
    }

    /// Returns the current organization display name.
    pub fn display_name(&self) -> Result<&OrganizationDisplayName, OrganizationError> {
        Ok(self.state_required()?.profile.display_name())
    }

    /// Returns the current organization display name.
    pub fn name(&self) -> Result<&OrganizationDisplayName, OrganizationError> {
        self.display_name()
    }

    /// Returns the current organization description.
    pub fn description(&self) -> Result<Option<&OrganizationDescription>, OrganizationError> {
        Ok(self.state_required()?.profile.description())
    }

    /// Returns the current organization website URL.
    pub fn website_url(&self) -> Result<Option<&OrganizationWebsiteUrl>, OrganizationError> {
        Ok(self.state_required()?.profile.website_url())
    }

    /// Returns the current organization picture.
    pub fn picture(&self) -> Result<Option<&OrganizationPictureRef>, OrganizationError> {
        Ok(self.state_required()?.profile.picture())
    }

    /// Returns the current organization handle.
    pub fn handle(&self) -> Result<&OrganizationHandle, OrganizationError> {
        Ok(&self.state_required()?.handle)
    }

    /// Returns the current organization owner.
    pub fn owner(&self) -> Result<OrganizationOwner, OrganizationError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the current organization status.
    pub fn status(&self) -> Result<OrganizationStatus, OrganizationError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the organization is active.
    pub fn is_active(&self) -> Result<bool, OrganizationError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the organization is removed.
    pub fn is_removed(&self) -> Result<bool, OrganizationError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Creates a new organization.
    pub fn create(
        &mut self,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        profile: OrganizationProfile,
    ) -> Result<(), OrganizationError> {
        if self.state().is_some() {
            return Err(OrganizationError::AlreadyCreated);
        }

        self.append_event(OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner,
            handle,
            profile,
        })
    }

    /// Changes the current organization handle.
    pub fn change_handle(&mut self, handle: OrganizationHandle) -> Result<(), OrganizationError> {
        self.ensure_not_removed()?;

        if self.state_required()?.handle == handle {
            return Ok(());
        }

        self.append_event(OrganizationEventPayload::HandleChanged { handle })
    }

    /// Changes the current organization profile.
    pub fn change_profile(
        &mut self,
        profile: OrganizationProfile,
    ) -> Result<(), OrganizationError> {
        self.ensure_not_removed()?;

        if self.state_required()?.profile == profile {
            return Ok(());
        }

        self.append_event(OrganizationEventPayload::ProfileChanged { profile })
    }

    /// Transfers ownership of the organization.
    pub fn transfer_ownership(
        &mut self,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationError> {
        self.ensure_not_removed()?;

        if self.state_required()?.owner == owner {
            return Ok(());
        }

        self.append_event(OrganizationEventPayload::OwnershipTransferred { owner })
    }

    /// Permanently removes the organization.
    pub fn remove(&mut self) -> Result<(), OrganizationError> {
        self.ensure_not_removed()?;

        self.append_event(OrganizationEventPayload::Removed)
    }

    fn ensure_not_removed(&self) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        Ok(())
    }
}

impl AggregateApply<OrganizationEventPayload, OrganizationError> for Organization {
    fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
        match payload {
            OrganizationEventPayload::Created {
                id,
                owner,
                handle,
                profile,
            } => self.set_state(Some(OrganizationState::new(
                *id,
                *owner,
                handle.clone(),
                profile.clone(),
            ))),
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
            }
            OrganizationEventPayload::ProfileChanged { profile } => {
                self.state_required_mut()?.profile = profile.clone();
            }
            OrganizationEventPayload::Removed => {
                self.state_required_mut()?.status = OrganizationStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use super::{
        Organization, OrganizationDescription, OrganizationDisplayName, OrganizationEventPayload,
        OrganizationHandle, OrganizationOwner, OrganizationPictureRef, OrganizationPictureUrl,
        OrganizationProfile, OrganizationWebsiteUrl,
    };

    fn owner() -> OrganizationOwner {
        OrganizationOwner::User(crate::UserId::new())
    }

    fn profile() -> OrganizationProfile {
        OrganizationProfile::new(
            OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid"),
            Some(
                OrganizationDescription::try_from("Independent research lab")
                    .expect("description should be valid"),
            ),
            Some(
                OrganizationWebsiteUrl::try_from("https://acme.example.com")
                    .expect("website URL should be valid"),
            ),
            Some(OrganizationPictureRef::external_url(
                OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                    .expect("picture URL should be valid"),
            )),
        )
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let mut organization = Organization::default();

        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationDisplayName::try_from("Acme Labs")
                        .expect("display name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");

        assert_eq!(
            organization
                .display_name()
                .expect("display name should exist")
                .value(),
            "Acme Labs"
        );
        assert_eq!(
            organization.uncommitted_events()[0].payload().name(),
            OrganizationEventPayload::CREATED
        );
    }

    #[test]
    fn change_profile_updates_state_and_records_event() {
        let mut organization = Organization::default();
        let profile = profile();
        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationDisplayName::try_from("Acme Labs")
                        .expect("display name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");

        organization
            .change_profile(profile.clone())
            .expect("profile change should succeed");

        assert_eq!(
            organization.profile().expect("profile should exist"),
            &profile
        );
        assert_eq!(
            organization.uncommitted_events()[1].payload().name(),
            OrganizationEventPayload::PROFILE_CHANGED
        );
    }

    #[test]
    fn same_profile_change_is_a_no_op() {
        let mut organization = Organization::default();
        let profile = profile();
        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationDisplayName::try_from("Acme Labs")
                        .expect("display name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");
        organization
            .change_profile(profile.clone())
            .expect("profile change should succeed");

        organization
            .change_profile(profile)
            .expect("idempotent profile change should succeed");

        assert_eq!(organization.uncommitted_events().len(), 2);
    }

    #[test]
    fn removed_organization_rejects_profile_changes() {
        let mut organization = Organization::default();
        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationProfile::new(
                    OrganizationDisplayName::try_from("Acme Labs")
                        .expect("display name should be valid"),
                    None,
                    None,
                    None,
                ),
            )
            .expect("creation should succeed");
        organization.remove().expect("remove should succeed");

        let error = organization
            .change_profile(profile())
            .expect_err("removed organization should reject changes");

        assert!(matches!(error, super::OrganizationError::Removed));
    }
}
