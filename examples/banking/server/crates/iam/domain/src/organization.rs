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
    core: AggregateCore<OrganizationId, OrganizationState, OrganizationEventPayload>,
}

impl Organization {
    /// Returns the current organization owner.
    pub fn owner(&self) -> Result<OrganizationOwner, OrganizationError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the current organization handle.
    pub fn handle(&self) -> Result<&OrganizationHandle, OrganizationError> {
        Ok(&self.state_required()?.handle)
    }

    /// Returns the current organization display name.
    pub fn display_name(&self) -> Result<&OrganizationDisplayName, OrganizationError> {
        Ok(&self.state_required()?.display_name)
    }

    /// Returns the current organization description.
    pub fn description(&self) -> Result<Option<&OrganizationDescription>, OrganizationError> {
        Ok(self.state_required()?.description.as_ref())
    }

    /// Returns the current organization website URL.
    pub fn website_url(&self) -> Result<Option<&OrganizationWebsiteUrl>, OrganizationError> {
        Ok(self.state_required()?.website_url.as_ref())
    }

    /// Returns the current organization picture.
    pub fn picture(&self) -> Result<Option<&OrganizationPictureRef>, OrganizationError> {
        Ok(self.state_required()?.picture.as_ref())
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
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationError> {
        if self.state().is_some() {
            return Err(OrganizationError::AlreadyCreated);
        }

        self.append_event(OrganizationEventPayload::Created {
            owner,
            handle,
            display_name,
        })?;

        Ok(())
    }

    /// Transfers ownership of the organization.
    pub fn transfer_ownership(
        &mut self,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::OwnershipTransferred { owner })?;
        Ok(())
    }

    /// Changes the current organization handle.
    pub fn change_handle(&mut self, handle: OrganizationHandle) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::HandleChanged { handle })?;
        Ok(())
    }

    /// Changes the current organization display name.
    pub fn change_display_name(
        &mut self,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::DisplayNameChanged { display_name })?;
        Ok(())
    }

    pub fn set_description(
        &mut self,
        description: Option<OrganizationDescription>,
    ) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::DescriptionSet { description })?;
        Ok(())
    }

    pub fn set_website_url(
        &mut self,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::WebsiteUrlSet { website_url })?;
        Ok(())
    }

    pub fn set_picture(
        &mut self,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        let old_picture = self.state_required()?.picture.clone();

        self.append_event(OrganizationEventPayload::PictureSet {
            picture,
            old_picture,
        })?;
        Ok(())
    }

    /// Permanently removes the organization.
    pub fn remove(&mut self) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationError::Removed);
        }

        self.append_event(OrganizationEventPayload::Removed)?;
        Ok(())
    }
}

impl AggregateApply<OrganizationEventPayload, OrganizationError> for Organization {
    fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
        match payload {
            OrganizationEventPayload::Created {
                owner,
                handle,
                display_name,
            } => self.set_state(Some(OrganizationState {
                owner: *owner,
                handle: handle.clone(),
                display_name: display_name.clone(),
                description: None,
                website_url: None,
                picture: None,
                status: OrganizationStatus::Active,
            })),
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
            }
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            OrganizationEventPayload::DescriptionSet { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            OrganizationEventPayload::WebsiteUrlSet { website_url } => {
                self.state_required_mut()?.website_url = website_url.clone();
            }
            OrganizationEventPayload::PictureSet { picture, .. } => {
                self.state_required_mut()?.picture = picture.clone();
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
        Organization, OrganizationDescription, OrganizationDisplayName, OrganizationError,
        OrganizationEventPayload, OrganizationHandle, OrganizationOwner, OrganizationPictureRef,
        OrganizationPictureUrl, OrganizationWebsiteUrl,
    };
    use crate::UserId;

    fn owner() -> OrganizationOwner {
        OrganizationOwner::User(UserId::new())
    }

    fn display_name() -> OrganizationDisplayName {
        OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid")
    }

    fn description() -> OrganizationDescription {
        OrganizationDescription::try_from("Independent research lab")
            .expect("description should be valid")
    }

    fn website_url() -> OrganizationWebsiteUrl {
        OrganizationWebsiteUrl::try_from("https://acme.example.com")
            .expect("website URL should be valid")
    }

    fn picture() -> OrganizationPictureRef {
        OrganizationPictureRef::external_url(
            OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                .expect("picture URL should be valid"),
        )
    }

    fn organization() -> Organization {
        let mut organization = Organization::new();
        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                display_name(),
            )
            .expect("creation should succeed");
        organization
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let organization = organization();

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
    fn create_rejects_already_created_organization_without_recording_an_event() {
        let mut organization = organization();
        let error = organization
            .create(
                owner(),
                OrganizationHandle::try_from("another-handle").expect("handle should be valid"),
                display_name(),
            )
            .expect_err("duplicate creation should fail");

        assert!(matches!(error, OrganizationError::AlreadyCreated));
        assert_eq!(organization.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_display_name_updates_state_and_records_event() {
        let mut organization = organization();
        let updated =
            OrganizationDisplayName::try_from("Acme Labs Updated").expect("name should be valid");

        organization
            .change_display_name(updated.clone())
            .expect("display name change should succeed");

        assert_eq!(
            organization
                .display_name()
                .expect("display name should exist"),
            &updated
        );
        assert_eq!(
            organization.uncommitted_events()[1].payload().name(),
            OrganizationEventPayload::DISPLAY_NAME_CHANGED
        );
    }

    #[test]
    fn same_display_name_change_appends_success_event() {
        let mut organization = organization();
        let updated =
            OrganizationDisplayName::try_from("Acme Labs Updated").expect("name should be valid");

        organization
            .change_display_name(updated.clone())
            .expect("display name change should succeed");
        organization
            .change_display_name(updated)
            .expect("repeated display name change should succeed");

        assert_eq!(organization.uncommitted_events().len(), 3);
    }

    #[test]
    fn set_description_website_url_and_picture_updates_state() {
        let mut organization = organization();

        organization
            .set_description(Some(description()))
            .expect("description change should succeed");
        organization
            .set_website_url(Some(website_url()))
            .expect("website URL change should succeed");
        organization
            .set_picture(Some(picture()))
            .expect("picture change should succeed");

        assert_eq!(
            organization
                .description()
                .expect("description should exist")
                .map(OrganizationDescription::value),
            Some("Independent research lab")
        );
        assert_eq!(
            organization
                .website_url()
                .expect("website URL should exist")
                .map(|value| value.value().as_str()),
            Some("https://acme.example.com/")
        );
        assert!(
            organization
                .picture()
                .expect("picture should exist")
                .is_some()
        );
    }

    #[test]
    fn picture_set_event_records_old_picture_after_current_picture() {
        let mut organization = organization();
        let first_picture = picture();
        let second_picture = OrganizationPictureRef::external_url(
            OrganizationPictureUrl::try_from("https://cdn.example.com/acme-updated.png")
                .expect("picture URL should be valid"),
        );
        organization
            .set_picture(Some(first_picture.clone()))
            .expect("picture change should succeed");

        organization
            .set_picture(Some(second_picture.clone()))
            .expect("picture change should succeed");

        let OrganizationEventPayload::PictureSet {
            picture,
            old_picture,
        } = organization.uncommitted_events()[2].payload()
        else {
            panic!("event should be picture changed");
        };
        assert_eq!(picture.as_ref(), Some(&second_picture));
        assert_eq!(old_picture.as_ref(), Some(&first_picture));
    }

    #[test]
    fn removed_organization_rejects_attribute_changes() {
        let mut organization = organization();
        organization.remove().expect("remove should succeed");

        let error = organization
            .set_description(Some(description()))
            .expect_err("removed organization should reject changes");

        assert!(matches!(error, OrganizationError::Removed));
    }
    #[test]
    fn optional_fields_start_empty_and_set_events_replay() {
        use appletheia::domain::AggregateApply;

        let mut organization = organization();
        assert!(organization.description().unwrap().is_none());
        assert!(organization.website_url().unwrap().is_none());
        assert!(organization.picture().unwrap().is_none());
        assert_eq!(organization.uncommitted_events().len(), 1);
        organization.set_description(Some(description())).unwrap();
        organization.set_website_url(Some(website_url())).unwrap();
        let previous_picture = picture();
        organization
            .set_picture(Some(previous_picture.clone()))
            .unwrap();
        organization.set_description(None).unwrap();
        organization.set_website_url(None).unwrap();
        organization.set_picture(None).unwrap();
        assert!(
            matches!(organization.uncommitted_events()[6].payload(), OrganizationEventPayload::PictureSet { picture: None, old_picture: Some(value) } if value == &previous_picture)
        );
        // Accepted repeated clears still emit events for downstream consumers.
        organization.set_description(None).unwrap();
        organization.set_website_url(None).unwrap();
        organization.set_picture(None).unwrap();
        assert_eq!(organization.uncommitted_events().len(), 10);

        let mut replayed = Organization::new();
        for event in organization.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), organization.state());
        assert!(replayed.description().unwrap().is_none());
        assert!(replayed.website_url().unwrap().is_none());
        assert!(replayed.picture().unwrap().is_none());
    }

    #[test]
    fn removed_organization_rejects_all_optional_field_operations() {
        let mut organization = organization();
        organization.remove().unwrap();
        for result in [
            organization.set_description(Some(description())),
            organization.set_description(None),
            organization.set_website_url(Some(website_url())),
            organization.set_website_url(None),
            organization.set_picture(Some(picture())),
            organization.set_picture(None),
        ] {
            assert!(matches!(result, Err(OrganizationError::Removed)));
        }
        assert_eq!(organization.uncommitted_events().len(), 2);
    }
}
