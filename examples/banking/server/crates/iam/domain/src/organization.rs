mod organization_error;
mod organization_event_payload;
mod organization_event_payload_error;
mod organization_id;
mod organization_name;
mod organization_name_error;
mod organization_state;
mod organization_state_error;

pub use organization_error::OrganizationError;
pub use organization_event_payload::OrganizationEventPayload;
pub use organization_event_payload_error::OrganizationEventPayloadError;
pub use organization_id::OrganizationId;
pub use organization_name::OrganizationName;
pub use organization_name_error::OrganizationNameError;
pub use organization_state::OrganizationState;
pub use organization_state_error::OrganizationStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `Organization` aggregate root.
#[aggregate(type = "organization", error = OrganizationError)]
pub struct Organization {
    core: AggregateCore<OrganizationState, OrganizationEventPayload>,
}

impl Organization {
    /// Returns the current organization name.
    pub fn name(&self) -> Result<&OrganizationName, OrganizationError> {
        Ok(&self.state_required()?.name)
    }

    /// Creates a new organization.
    pub fn create(&mut self, name: OrganizationName) -> Result<(), OrganizationError> {
        if self.state().is_some() {
            return Err(OrganizationError::AlreadyCreated);
        }

        self.append_event(OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            name,
        })
    }
}

impl AggregateApply<OrganizationEventPayload, OrganizationError> for Organization {
    fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
        match payload {
            OrganizationEventPayload::Created { id, name } => {
                self.set_state(Some(OrganizationState::new(*id, name.clone())));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{Organization, OrganizationEventPayload, OrganizationName};

    #[test]
    fn create_initializes_state_and_records_event() {
        let name = OrganizationName::try_from("  Acme Labs  ").expect("name should be valid");
        let mut organization = Organization::default();

        organization
            .create(name.clone())
            .expect("creation should succeed");

        let aggregate_id = organization
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
        assert_eq!(organization.name().expect("name should exist"), &name);
        assert_eq!(organization.uncommitted_events().len(), 1);
        assert_eq!(
            organization.uncommitted_events()[0].payload().name(),
            OrganizationEventPayload::CREATED
        );
    }

    #[test]
    fn creating_twice_returns_an_error() {
        let mut organization = Organization::default();
        organization
            .create(OrganizationName::try_from("Acme Labs").expect("name should be valid"))
            .expect("first creation should succeed");

        let error = organization
            .create(OrganizationName::try_from("Second").expect("name should be valid"))
            .expect_err("second creation should fail");

        assert!(matches!(error, super::OrganizationError::AlreadyCreated));
    }
}
