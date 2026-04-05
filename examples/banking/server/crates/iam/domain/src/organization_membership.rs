mod organization_membership_error;
mod organization_membership_event_payload;
mod organization_membership_event_payload_error;
mod organization_membership_id;
mod organization_membership_state;
mod organization_membership_state_error;
mod organization_membership_status;

pub use organization_membership_error::OrganizationMembershipError;
pub use organization_membership_event_payload::OrganizationMembershipEventPayload;
pub use organization_membership_event_payload_error::OrganizationMembershipEventPayloadError;
pub use organization_membership_id::OrganizationMembershipId;
pub use organization_membership_state::OrganizationMembershipState;
pub use organization_membership_state_error::OrganizationMembershipStateError;
pub use organization_membership_status::OrganizationMembershipStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationId, UserId};

/// Represents the `OrganizationMembership` aggregate root.
#[aggregate(type = "organization_membership", error = OrganizationMembershipError)]
pub struct OrganizationMembership {
    core: AggregateCore<OrganizationMembershipState, OrganizationMembershipEventPayload>,
}

impl OrganizationMembership {
    /// Returns the organization this membership belongs to.
    pub fn organization_id(&self) -> Result<&OrganizationId, OrganizationMembershipError> {
        Ok(&self.state_required()?.organization_id)
    }

    /// Returns the user this membership belongs to.
    pub fn user_id(&self) -> Result<&UserId, OrganizationMembershipError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the current membership status.
    pub fn status(&self) -> Result<OrganizationMembershipStatus, OrganizationMembershipError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the membership is active.
    pub fn is_active(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the membership is inactive.
    pub fn is_inactive(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_inactive())
    }

    /// Returns whether the membership is removed.
    pub fn is_removed(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Creates a new organization membership.
    pub fn create(
        &mut self,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state().is_some() {
            return Err(OrganizationMembershipError::AlreadyCreated);
        }

        self.append_event(OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id,
            user_id,
        })
    }

    /// Activates an inactive membership.
    pub fn activate(&mut self) -> Result<(), OrganizationMembershipError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_active() {
            return Ok(());
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Activated {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })
    }

    /// Deactivates an active membership.
    pub fn deactivate(&mut self) -> Result<(), OrganizationMembershipError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_inactive() {
            return Ok(());
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Inactivated {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })
    }

    /// Permanently removes the membership.
    pub fn remove(&mut self) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Ok(());
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Removed {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })
    }

    fn ensure_not_removed(&self) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationMembershipError::Removed);
        }

        Ok(())
    }
}

impl AggregateApply<OrganizationMembershipEventPayload, OrganizationMembershipError>
    for OrganizationMembership
{
    fn apply(
        &mut self,
        payload: &OrganizationMembershipEventPayload,
    ) -> Result<(), OrganizationMembershipError> {
        match payload {
            OrganizationMembershipEventPayload::Created {
                id,
                organization_id,
                user_id,
            } => {
                self.set_state(Some(OrganizationMembershipState::new(
                    *id,
                    *organization_id,
                    *user_id,
                )));
            }
            OrganizationMembershipEventPayload::Activated { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Active;
            }
            OrganizationMembershipEventPayload::Inactivated { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Inactive;
            }
            OrganizationMembershipEventPayload::Removed { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipStatus,
    };
    use crate::{OrganizationId, UserId};

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    fn user_id() -> UserId {
        UserId::new()
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let organization_id = organization_id();
        let user_id = user_id();
        let mut membership = OrganizationMembership::default();

        membership
            .create(organization_id, user_id)
            .expect("creation should succeed");

        let aggregate_id = membership
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
        assert_eq!(
            membership
                .organization_id()
                .expect("organization id should exist"),
            &organization_id
        );
        assert_eq!(
            membership.user_id().expect("user id should exist"),
            &user_id
        );
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Active
        );
        assert_eq!(membership.uncommitted_events().len(), 1);
        assert_eq!(
            membership.uncommitted_events()[0].payload().name(),
            OrganizationMembershipEventPayload::CREATED
        );
    }

    #[test]
    fn activate_and_deactivate_update_status_and_record_events() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");

        membership.deactivate().expect("deactivate should succeed");
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Inactive
        );

        membership.activate().expect("activate should succeed");
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Active
        );
        assert_eq!(membership.uncommitted_events().len(), 3);
        assert_eq!(
            membership.uncommitted_events()[1].payload(),
            &OrganizationMembershipEventPayload::Inactivated {
                organization_id: organization_id_value,
                user_id: user_id_value,
            }
        );
        assert_eq!(
            membership.uncommitted_events()[2].payload(),
            &OrganizationMembershipEventPayload::Activated {
                organization_id: organization_id_value,
                user_id: user_id_value,
            }
        );
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");

        membership.remove().expect("remove should succeed");

        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Removed
        );
        assert_eq!(membership.uncommitted_events().len(), 2);
        assert_eq!(
            membership.uncommitted_events()[1].payload(),
            &OrganizationMembershipEventPayload::Removed {
                organization_id: organization_id_value,
                user_id: user_id_value,
            }
        );
    }

    #[test]
    fn removed_membership_rejects_status_changes() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id(), user_id())
            .expect("creation should succeed");
        membership.remove().expect("remove should succeed");

        let activate_error = membership.activate().expect_err("activate should fail");
        assert!(matches!(
            activate_error,
            super::OrganizationMembershipError::Removed
        ));

        let deactivate_error = membership.deactivate().expect_err("deactivate should fail");
        assert!(matches!(
            deactivate_error,
            super::OrganizationMembershipError::Removed
        ));
    }
}
