mod organization_role;
mod organization_role_assignee;
mod organization_role_assignee_error;
mod organization_role_assignment_error;
mod organization_role_assignment_event_payload;
mod organization_role_assignment_event_payload_error;
mod organization_role_assignment_id;
mod organization_role_assignment_state;
mod organization_role_assignment_state_error;
mod organization_role_assignment_status;

pub use organization_role::*;
pub use organization_role_assignee::*;
pub use organization_role_assignee_error::*;
pub use organization_role_assignment_error::*;
pub use organization_role_assignment_event_payload::*;
pub use organization_role_assignment_event_payload_error::*;
pub use organization_role_assignment_id::*;
pub use organization_role_assignment_state::*;
pub use organization_role_assignment_state_error::*;
pub use organization_role_assignment_status::*;

use crate::OrganizationId;
use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

#[aggregate(type = "organization_role_assignment", error = OrganizationRoleAssignmentError)]
pub struct OrganizationRoleAssignment {
    core: AggregateCore<
        OrganizationRoleAssignmentId,
        OrganizationRoleAssignmentState,
        OrganizationRoleAssignmentEventPayload,
    >,
}

impl OrganizationRoleAssignment {
    pub fn organization_id(&self) -> Result<OrganizationId, OrganizationRoleAssignmentError> {
        Ok(self.state_required()?.organization_id)
    }

    pub fn assignee(&self) -> Result<OrganizationRoleAssignee, OrganizationRoleAssignmentError> {
        Ok(self.state_required()?.assignee)
    }

    pub fn role(&self) -> Result<OrganizationRole, OrganizationRoleAssignmentError> {
        Ok(self.state_required()?.role)
    }

    pub fn status(
        &self,
    ) -> Result<OrganizationRoleAssignmentStatus, OrganizationRoleAssignmentError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, OrganizationRoleAssignmentError> {
        Ok(self.status()? == OrganizationRoleAssignmentStatus::Active)
    }

    pub fn assign(
        &mut self,
        organization_id: OrganizationId,
        assignee: OrganizationRoleAssignee,
        role: OrganizationRole,
    ) -> Result<(), OrganizationRoleAssignmentError> {
        if self.state().is_some() {
            return Err(OrganizationRoleAssignmentError::AlreadyAssigned);
        }
        self.append_event(OrganizationRoleAssignmentEventPayload::Assigned {
            organization_id,
            assignee,
            role,
        })?;
        Ok(())
    }

    pub fn revoke(&mut self) -> Result<(), OrganizationRoleAssignmentError> {
        if !self.is_active()? {
            return Err(OrganizationRoleAssignmentError::Revoked);
        }
        self.append_event(OrganizationRoleAssignmentEventPayload::Revoked)?;
        Ok(())
    }
}

impl AggregateApply<OrganizationRoleAssignmentEventPayload, OrganizationRoleAssignmentError>
    for OrganizationRoleAssignment
{
    fn apply(
        &mut self,
        payload: &OrganizationRoleAssignmentEventPayload,
    ) -> Result<(), OrganizationRoleAssignmentError> {
        match payload {
            OrganizationRoleAssignmentEventPayload::Assigned {
                organization_id,
                assignee,
                role,
            } => self.set_state(Some(OrganizationRoleAssignmentState {
                organization_id: *organization_id,
                assignee: *assignee,
                role: *role,
                status: OrganizationRoleAssignmentStatus::Active,
            })),
            OrganizationRoleAssignmentEventPayload::Revoked => {
                self.state_required_mut()?.status = OrganizationRoleAssignmentStatus::Revoked
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OrganizationMembershipId;

    #[test]
    fn assignment_lifecycle_preserves_history_and_releases_uniqueness() {
        let scope_id = OrganizationId::new();
        let assignee =
            OrganizationRoleAssignee::OrganizationMembership(OrganizationMembershipId::new());
        let mut assignment = OrganizationRoleAssignment::new();
        assignment
            .assign(scope_id, assignee, OrganizationRole::Admin)
            .unwrap();
        assert!(!assignment.unique_entries().unwrap().is_empty());
        assert!(matches!(
            assignment.assign(scope_id, assignee, OrganizationRole::Admin),
            Err(OrganizationRoleAssignmentError::AlreadyAssigned)
        ));
        assignment.revoke().unwrap();
        assert!(assignment.unique_entries().unwrap().is_empty());
        assert!(matches!(
            assignment.revoke(),
            Err(OrganizationRoleAssignmentError::Revoked)
        ));
        assert!(matches!(
            assignment.assign(scope_id, assignee, OrganizationRole::Admin),
            Err(OrganizationRoleAssignmentError::AlreadyAssigned)
        ));
        let mut restored = OrganizationRoleAssignment::new();
        for event in assignment.uncommitted_events() {
            restored.apply(event.payload()).unwrap();
        }
        assert_eq!(restored.state(), assignment.state());
        assert_ne!(
            assignee
                .assignment_unique_value(scope_id, OrganizationRole::Admin)
                .unwrap(),
            assignee
                .assignment_unique_value(OrganizationId::new(), OrganizationRole::Admin)
                .unwrap()
        );
    }
}
