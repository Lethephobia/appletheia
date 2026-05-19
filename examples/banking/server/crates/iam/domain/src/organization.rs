mod organization_description;
mod organization_description_change_rejection_reason;
mod organization_description_change_result;
mod organization_description_error;
mod organization_display_name;
mod organization_display_name_change_rejection_reason;
mod organization_display_name_change_result;
mod organization_display_name_error;
mod organization_error;
mod organization_event_payload;
mod organization_event_payload_error;
mod organization_handle;
mod organization_handle_change_rejection_reason;
mod organization_handle_change_result;
mod organization_handle_error;
mod organization_id;
mod organization_owner;
mod organization_ownership_transfer_rejection_reason;
mod organization_ownership_transfer_result;
mod organization_picture_change_rejection_reason;
mod organization_picture_change_result;
mod organization_picture_object_name;
mod organization_picture_object_name_error;
mod organization_picture_ref;
mod organization_picture_url;
mod organization_picture_url_error;
mod organization_remove_rejection_reason;
mod organization_remove_result;
mod organization_state;
mod organization_state_error;
mod organization_status;
mod organization_website_url;
mod organization_website_url_change_rejection_reason;
mod organization_website_url_change_result;
mod organization_website_url_error;

pub use organization_description::OrganizationDescription;
pub use organization_description_change_rejection_reason::OrganizationDescriptionChangeRejectionReason;
pub use organization_description_change_result::OrganizationDescriptionChangeResult;
pub use organization_description_error::OrganizationDescriptionError;
pub use organization_display_name::OrganizationDisplayName;
pub use organization_display_name_change_rejection_reason::OrganizationDisplayNameChangeRejectionReason;
pub use organization_display_name_change_result::OrganizationDisplayNameChangeResult;
pub use organization_display_name_error::OrganizationDisplayNameError;
pub use organization_error::OrganizationError;
pub use organization_event_payload::OrganizationEventPayload;
pub use organization_event_payload_error::OrganizationEventPayloadError;
pub use organization_handle::OrganizationHandle;
pub use organization_handle_change_rejection_reason::OrganizationHandleChangeRejectionReason;
pub use organization_handle_change_result::OrganizationHandleChangeResult;
pub use organization_handle_error::OrganizationHandleError;
pub use organization_id::OrganizationId;
pub use organization_owner::OrganizationOwner;
pub use organization_ownership_transfer_rejection_reason::OrganizationOwnershipTransferRejectionReason;
pub use organization_ownership_transfer_result::OrganizationOwnershipTransferResult;
pub use organization_picture_change_rejection_reason::OrganizationPictureChangeRejectionReason;
pub use organization_picture_change_result::OrganizationPictureChangeResult;
pub use organization_picture_object_name::OrganizationPictureObjectName;
pub use organization_picture_object_name_error::OrganizationPictureObjectNameError;
pub use organization_picture_ref::OrganizationPictureRef;
pub use organization_picture_url::OrganizationPictureUrl;
pub use organization_picture_url_error::OrganizationPictureUrlError;
pub use organization_remove_rejection_reason::OrganizationRemoveRejectionReason;
pub use organization_remove_result::OrganizationRemoveResult;
pub use organization_state::OrganizationState;
pub use organization_state_error::OrganizationStateError;
pub use organization_status::OrganizationStatus;
pub use organization_website_url::OrganizationWebsiteUrl;
pub use organization_website_url_change_rejection_reason::OrganizationWebsiteUrlChangeRejectionReason;
pub use organization_website_url_change_result::OrganizationWebsiteUrlChangeResult;
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
        description: Option<OrganizationDescription>,
        website_url: Option<OrganizationWebsiteUrl>,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationError> {
        if self.state().is_some() {
            return Err(OrganizationError::AlreadyCreated);
        }

        self.append_event(OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner,
            handle,
            display_name,
            description,
            website_url,
            picture,
        })
    }

    /// Transfers ownership of the organization.
    pub fn transfer_ownership(
        &mut self,
        owner: OrganizationOwner,
    ) -> Result<OrganizationOwnershipTransferResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationOwnershipTransferRejectionReason::Removed;
            return self.reject_transfer_ownership(owner, reason);
        }

        self.append_event(OrganizationEventPayload::OwnershipTransferred { owner })?;
        Ok(OrganizationOwnershipTransferResult::Transferred)
    }

    /// Rejects an ownership transfer attempt.
    pub fn reject_transfer_ownership(
        &mut self,
        owner: OrganizationOwner,
        reason: OrganizationOwnershipTransferRejectionReason,
    ) -> Result<OrganizationOwnershipTransferResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::OwnershipTransferRejected { owner, reason })?;
        Ok(OrganizationOwnershipTransferResult::Rejected { reason })
    }

    /// Changes the current organization handle.
    pub fn change_handle(
        &mut self,
        handle: OrganizationHandle,
    ) -> Result<OrganizationHandleChangeResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationHandleChangeRejectionReason::Removed;
            return self.reject_change_handle(handle, reason);
        }

        self.append_event(OrganizationEventPayload::HandleChanged { handle })?;
        Ok(OrganizationHandleChangeResult::Changed)
    }

    /// Rejects a handle change attempt.
    pub fn reject_change_handle(
        &mut self,
        handle: OrganizationHandle,
        reason: OrganizationHandleChangeRejectionReason,
    ) -> Result<OrganizationHandleChangeResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::HandleChangeRejected { handle, reason })?;
        Ok(OrganizationHandleChangeResult::Rejected { reason })
    }

    /// Changes the current organization display name.
    pub fn change_display_name(
        &mut self,
        display_name: OrganizationDisplayName,
    ) -> Result<OrganizationDisplayNameChangeResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationDisplayNameChangeRejectionReason::Removed;
            return self.reject_change_display_name(display_name, reason);
        }

        self.append_event(OrganizationEventPayload::DisplayNameChanged { display_name })?;
        Ok(OrganizationDisplayNameChangeResult::Changed)
    }

    /// Rejects a display name change attempt.
    pub fn reject_change_display_name(
        &mut self,
        display_name: OrganizationDisplayName,
        reason: OrganizationDisplayNameChangeRejectionReason,
    ) -> Result<OrganizationDisplayNameChangeResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::DisplayNameChangeRejected {
            display_name,
            reason,
        })?;
        Ok(OrganizationDisplayNameChangeResult::Rejected { reason })
    }

    /// Changes the current organization description.
    pub fn change_description(
        &mut self,
        description: Option<OrganizationDescription>,
    ) -> Result<OrganizationDescriptionChangeResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationDescriptionChangeRejectionReason::Removed;
            return self.reject_change_description(description, reason);
        }

        self.append_event(OrganizationEventPayload::DescriptionChanged { description })?;
        Ok(OrganizationDescriptionChangeResult::Changed)
    }

    /// Rejects a description change attempt.
    pub fn reject_change_description(
        &mut self,
        description: Option<OrganizationDescription>,
        reason: OrganizationDescriptionChangeRejectionReason,
    ) -> Result<OrganizationDescriptionChangeResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::DescriptionChangeRejected {
            description,
            reason,
        })?;
        Ok(OrganizationDescriptionChangeResult::Rejected { reason })
    }

    /// Changes the current organization website URL.
    pub fn change_website_url(
        &mut self,
        website_url: Option<OrganizationWebsiteUrl>,
    ) -> Result<OrganizationWebsiteUrlChangeResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationWebsiteUrlChangeRejectionReason::Removed;
            return self.reject_change_website_url(website_url, reason);
        }

        self.append_event(OrganizationEventPayload::WebsiteUrlChanged { website_url })?;
        Ok(OrganizationWebsiteUrlChangeResult::Changed)
    }

    /// Rejects a website URL change attempt.
    pub fn reject_change_website_url(
        &mut self,
        website_url: Option<OrganizationWebsiteUrl>,
        reason: OrganizationWebsiteUrlChangeRejectionReason,
    ) -> Result<OrganizationWebsiteUrlChangeResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::WebsiteUrlChangeRejected {
            website_url,
            reason,
        })?;
        Ok(OrganizationWebsiteUrlChangeResult::Rejected { reason })
    }

    /// Changes the current organization picture.
    pub fn change_picture(
        &mut self,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<OrganizationPictureChangeResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationPictureChangeRejectionReason::Removed;
            return self.reject_change_picture(picture, reason);
        }

        let old_picture = self.state_required()?.picture.clone();

        self.append_event(OrganizationEventPayload::PictureChanged {
            picture,
            old_picture,
        })?;
        Ok(OrganizationPictureChangeResult::Changed)
    }

    /// Rejects a picture change attempt.
    pub fn reject_change_picture(
        &mut self,
        picture: Option<OrganizationPictureRef>,
        reason: OrganizationPictureChangeRejectionReason,
    ) -> Result<OrganizationPictureChangeResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::PictureChangeRejected { picture, reason })?;
        Ok(OrganizationPictureChangeResult::Rejected { reason })
    }

    /// Permanently removes the organization.
    pub fn remove(&mut self) -> Result<OrganizationRemoveResult, OrganizationError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationRemoveRejectionReason::Removed;
            return self.reject_remove(reason);
        }

        self.append_event(OrganizationEventPayload::Removed)?;
        Ok(OrganizationRemoveResult::Removed)
    }

    /// Rejects an organization removal attempt.
    pub fn reject_remove(
        &mut self,
        reason: OrganizationRemoveRejectionReason,
    ) -> Result<OrganizationRemoveResult, OrganizationError> {
        self.append_event(OrganizationEventPayload::RemoveRejected { reason })?;
        Ok(OrganizationRemoveResult::Rejected { reason })
    }
}

impl AggregateApply<OrganizationEventPayload, OrganizationError> for Organization {
    fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
        match payload {
            OrganizationEventPayload::Created {
                id,
                owner,
                handle,
                display_name,
                description,
                website_url,
                picture,
            } => self.set_state(Some(OrganizationState {
                id: *id,
                owner: *owner,
                handle: handle.clone(),
                display_name: display_name.clone(),
                description: description.clone(),
                website_url: website_url.clone(),
                picture: picture.clone(),
                status: OrganizationStatus::Active,
            })),
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            OrganizationEventPayload::OwnershipTransferRejected { .. } => {}
            OrganizationEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
            }
            OrganizationEventPayload::HandleChangeRejected { .. } => {}
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            OrganizationEventPayload::DisplayNameChangeRejected { .. } => {}
            OrganizationEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            OrganizationEventPayload::DescriptionChangeRejected { .. } => {}
            OrganizationEventPayload::WebsiteUrlChanged { website_url } => {
                self.state_required_mut()?.website_url = website_url.clone();
            }
            OrganizationEventPayload::WebsiteUrlChangeRejected { .. } => {}
            OrganizationEventPayload::PictureChanged { picture, .. } => {
                self.state_required_mut()?.picture = picture.clone();
            }
            OrganizationEventPayload::PictureChangeRejected { .. } => {}
            OrganizationEventPayload::Removed => {
                self.state_required_mut()?.status = OrganizationStatus::Removed;
            }
            OrganizationEventPayload::RemoveRejected { .. } => {}
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
        OrganizationWebsiteUrl,
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
        let mut organization = Organization::default();
        organization
            .create(
                owner(),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                display_name(),
                None,
                None,
                None,
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
    fn change_description_website_url_and_picture_updates_state() {
        let mut organization = organization();

        organization
            .change_description(Some(description()))
            .expect("description change should succeed");
        organization
            .change_website_url(Some(website_url()))
            .expect("website URL change should succeed");
        organization
            .change_picture(Some(picture()))
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
    fn picture_changed_event_records_old_picture_after_current_picture() {
        let mut organization = organization();
        let first_picture = picture();
        let second_picture = OrganizationPictureRef::external_url(
            OrganizationPictureUrl::try_from("https://cdn.example.com/acme-updated.png")
                .expect("picture URL should be valid"),
        );
        organization
            .change_picture(Some(first_picture.clone()))
            .expect("picture change should succeed");

        organization
            .change_picture(Some(second_picture.clone()))
            .expect("picture change should succeed");

        let OrganizationEventPayload::PictureChanged {
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

        let result = organization
            .change_description(Some(description()))
            .expect("removed organization should reject changes with an event");

        assert!(matches!(
            result,
            super::OrganizationDescriptionChangeResult::Rejected {
                reason: super::OrganizationDescriptionChangeRejectionReason::Removed
            }
        ));
    }
}
