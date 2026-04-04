mod organization_error;
mod organization_event_payload;
mod organization_event_payload_error;
mod organization_handle;
mod organization_handle_error;
mod organization_id;
mod organization_name;
mod organization_name_error;
mod organization_state;
mod organization_state_error;
mod organization_status;

pub use organization_error::OrganizationError;
pub use organization_event_payload::OrganizationEventPayload;
pub use organization_event_payload_error::OrganizationEventPayloadError;
pub use organization_handle::OrganizationHandle;
pub use organization_handle_error::OrganizationHandleError;
pub use organization_id::OrganizationId;
pub use organization_name::OrganizationName;
pub use organization_name_error::OrganizationNameError;
pub use organization_state::OrganizationState;
pub use organization_state_error::OrganizationStateError;
pub use organization_status::OrganizationStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `Organization` aggregate root.
#[aggregate(type = "organization", error = OrganizationError)]
pub struct Organization {
    core: AggregateCore<OrganizationState, OrganizationEventPayload>,
}

impl Organization {
    /// Returns the current organization handle.
    pub fn handle(&self) -> Result<&OrganizationHandle, OrganizationError> {
        Ok(&self.state_required()?.handle)
    }

    /// Returns the current organization name.
    pub fn name(&self) -> Result<&OrganizationName, OrganizationError> {
        Ok(&self.state_required()?.name)
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
        handle: OrganizationHandle,
        name: OrganizationName,
    ) -> Result<(), OrganizationError> {
        if self.state().is_some() {
            return Err(OrganizationError::AlreadyCreated);
        }

        self.append_event(OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            handle,
            name,
        })
    }

    /// Changes the current organization handle.
    pub fn change_handle(&mut self, handle: OrganizationHandle) -> Result<(), OrganizationError> {
        self.ensure_not_removed()?;

        let current_handle = self.state_required()?.handle.clone();

        if current_handle.eq(&handle) {
            return Ok(());
        }

        self.append_event(OrganizationEventPayload::HandleChanged { handle })
    }

    /// Permanently removes the organization.
    pub fn remove(&mut self) -> Result<(), OrganizationError> {
        if self.state_required()?.status.is_removed() {
            return Ok(());
        }

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
            OrganizationEventPayload::Created { id, handle, name } => {
                self.set_state(Some(OrganizationState::new(
                    *id,
                    handle.clone(),
                    name.clone(),
                )));
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
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
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{Organization, OrganizationEventPayload, OrganizationHandle, OrganizationName};

    #[test]
    fn create_initializes_state_and_records_event() {
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let name = OrganizationName::try_from("  Acme Labs  ").expect("name should be valid");
        let mut organization = Organization::default();

        organization
            .create(handle.clone(), name.clone())
            .expect("creation should succeed");

        let aggregate_id = organization
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
        assert_eq!(organization.handle().expect("handle should exist"), &handle);
        assert_eq!(organization.name().expect("name should exist"), &name);
        assert_eq!(organization.uncommitted_events().len(), 1);
        assert_eq!(
            organization.uncommitted_events()[0].payload().name(),
            OrganizationEventPayload::CREATED
        );
    }

    #[test]
    fn changing_handle_updates_state_and_records_event() {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("first creation should succeed");

        organization
            .change_handle(
                OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid"),
            )
            .expect("handle change should succeed");

        assert_eq!(
            organization.handle().expect("handle should exist"),
            &OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid")
        );
        assert_eq!(organization.uncommitted_events().len(), 2);
        assert_eq!(
            organization.uncommitted_events()[1].payload().name(),
            OrganizationEventPayload::HANDLE_CHANGED
        );
    }

    #[test]
    fn removing_organization_updates_status_and_records_event() {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("first creation should succeed");

        organization.remove().expect("remove should succeed");

        assert!(organization.is_removed().expect("status should exist"));
        assert_eq!(organization.uncommitted_events().len(), 2);
        assert_eq!(
            organization.uncommitted_events()[1].payload().name(),
            OrganizationEventPayload::REMOVED
        );
    }

    #[test]
    fn removed_organization_rejects_handle_changes() {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("first creation should succeed");
        organization.remove().expect("remove should succeed");

        let error = organization
            .change_handle(
                OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid"),
            )
            .expect_err("removed organization should reject changes");

        assert!(matches!(error, super::OrganizationError::Removed));
    }

    #[test]
    fn changing_to_same_handle_is_a_no_op() {
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let mut organization = Organization::default();
        organization
            .create(
                handle.clone(),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("first creation should succeed");

        organization
            .change_handle(handle)
            .expect("idempotent change should succeed");

        assert_eq!(organization.uncommitted_events().len(), 1);
    }

    #[test]
    fn creating_twice_returns_an_error() {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationName::try_from("Acme Labs").expect("name should be valid"),
            )
            .expect("first creation should succeed");

        let error = organization
            .create(
                OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid"),
                OrganizationName::try_from("Second").expect("name should be valid"),
            )
            .expect_err("second creation should fail");

        assert!(matches!(error, super::OrganizationError::AlreadyCreated));
    }
}
