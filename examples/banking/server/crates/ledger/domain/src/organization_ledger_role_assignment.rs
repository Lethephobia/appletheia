mod organization_ledger_role;
mod organization_ledger_role_assignee;
mod organization_ledger_role_assignee_error;
mod organization_ledger_role_assignment_error;
mod organization_ledger_role_assignment_event_payload;
mod organization_ledger_role_assignment_event_payload_error;
mod organization_ledger_role_assignment_id;
mod organization_ledger_role_assignment_state;
mod organization_ledger_role_assignment_state_error;
mod organization_ledger_role_assignment_status;

pub use organization_ledger_role::*;
pub use organization_ledger_role_assignee::*;
pub use organization_ledger_role_assignee_error::*;
pub use organization_ledger_role_assignment_error::*;
pub use organization_ledger_role_assignment_event_payload::*;
pub use organization_ledger_role_assignment_event_payload_error::*;
pub use organization_ledger_role_assignment_id::*;
pub use organization_ledger_role_assignment_state::*;
pub use organization_ledger_role_assignment_state_error::*;
pub use organization_ledger_role_assignment_status::*;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
use banking_iam_domain::OrganizationId;

#[aggregate(type = "organization_ledger_role_assignment", error = OrganizationLedgerRoleAssignmentError)]
pub struct OrganizationLedgerRoleAssignment {
    core: AggregateCore<
        OrganizationLedgerRoleAssignmentId,
        OrganizationLedgerRoleAssignmentState,
        OrganizationLedgerRoleAssignmentEventPayload,
    >,
}

impl OrganizationLedgerRoleAssignment {
    pub fn organization_id(&self) -> Result<OrganizationId, OrganizationLedgerRoleAssignmentError> {
        Ok(self.state_required()?.organization_id)
    }

    pub fn assignee(
        &self,
    ) -> Result<OrganizationLedgerRoleAssignee, OrganizationLedgerRoleAssignmentError> {
        Ok(self.state_required()?.assignee)
    }

    pub fn role(&self) -> Result<OrganizationLedgerRole, OrganizationLedgerRoleAssignmentError> {
        Ok(self.state_required()?.role)
    }

    pub fn status(
        &self,
    ) -> Result<OrganizationLedgerRoleAssignmentStatus, OrganizationLedgerRoleAssignmentError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, OrganizationLedgerRoleAssignmentError> {
        Ok(self.status()? == OrganizationLedgerRoleAssignmentStatus::Active)
    }

    pub fn assign(
        &mut self,
        organization_id: OrganizationId,
        assignee: OrganizationLedgerRoleAssignee,
        role: OrganizationLedgerRole,
    ) -> Result<(), OrganizationLedgerRoleAssignmentError> {
        if self.state().is_some() {
            return Err(OrganizationLedgerRoleAssignmentError::AlreadyAssigned);
        }
        self.append_event(OrganizationLedgerRoleAssignmentEventPayload::Assigned {
            organization_id,
            assignee,
            role,
        })?;
        Ok(())
    }

    pub fn unassign(&mut self) -> Result<(), OrganizationLedgerRoleAssignmentError> {
        if !self.is_active()? {
            return Err(OrganizationLedgerRoleAssignmentError::Unassigned);
        }
        self.append_event(OrganizationLedgerRoleAssignmentEventPayload::Unassigned)?;
        Ok(())
    }
}

impl
    AggregateApply<
        OrganizationLedgerRoleAssignmentEventPayload,
        OrganizationLedgerRoleAssignmentError,
    > for OrganizationLedgerRoleAssignment
{
    fn apply(
        &mut self,
        payload: &OrganizationLedgerRoleAssignmentEventPayload,
    ) -> Result<(), OrganizationLedgerRoleAssignmentError> {
        match payload {
            OrganizationLedgerRoleAssignmentEventPayload::Assigned {
                organization_id,
                assignee,
                role,
            } => self.set_state(Some(OrganizationLedgerRoleAssignmentState {
                organization_id: *organization_id,
                assignee: *assignee,
                role: *role,
                status: OrganizationLedgerRoleAssignmentStatus::Active,
            })),
            OrganizationLedgerRoleAssignmentEventPayload::Unassigned => {
                self.state_required_mut()?.status =
                    OrganizationLedgerRoleAssignmentStatus::Unassigned
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia::domain::{AggregateId, UniqueConstraints};
    use banking_iam_domain::{OrganizationMembershipId, TeamId};

    #[test]
    fn unassign_releases_unique_assignment_and_cannot_be_reactivated() {
        let organization_id = OrganizationId::new();
        let assignee =
            OrganizationLedgerRoleAssignee::OrganizationMembership(OrganizationMembershipId::new());
        let mut assignment = OrganizationLedgerRoleAssignment::new();
        assignment
            .assign(
                organization_id,
                assignee,
                OrganizationLedgerRole::FinanceManager,
            )
            .unwrap();
        let before = assignment
            .state()
            .unwrap()
            .unique_entries(assignment.aggregate_id().value())
            .unwrap();
        assert!(!before.is_empty());
        assignment.unassign().unwrap();
        assert!(!assignment.is_active().unwrap());
        assert!(
            assignment
                .state()
                .unwrap()
                .unique_entries(assignment.aggregate_id().value())
                .unwrap()
                .is_empty()
        );
        assert!(matches!(
            assignment.unassign(),
            Err(OrganizationLedgerRoleAssignmentError::Unassigned)
        ));
        assert!(matches!(
            assignment.assign(organization_id, assignee, OrganizationLedgerRole::Treasurer),
            Err(OrganizationLedgerRoleAssignmentError::AlreadyAssigned)
        ));
        let mut restored = OrganizationLedgerRoleAssignment::new();
        for event in assignment.uncommitted_events() {
            restored.apply(event.payload()).unwrap();
        }
        assert_eq!(restored.state(), assignment.state());
    }

    #[test]
    fn unique_key_distinguishes_roles_organizations_and_assignee_kinds() {
        let organization_id = OrganizationId::new();
        let membership_id = OrganizationMembershipId::new();
        let member = OrganizationLedgerRoleAssignee::OrganizationMembership(membership_id);
        let team = OrganizationLedgerRoleAssignee::Team(
            TeamId::try_from_uuid(membership_id.value()).unwrap(),
        );
        let key = member
            .assignment_unique_value(organization_id, OrganizationLedgerRole::FinanceManager)
            .unwrap();
        assert_ne!(
            key,
            member
                .assignment_unique_value(organization_id, OrganizationLedgerRole::Treasurer)
                .unwrap()
        );
        assert_ne!(
            key,
            member
                .assignment_unique_value(
                    OrganizationId::new(),
                    OrganizationLedgerRole::FinanceManager
                )
                .unwrap()
        );
        assert_ne!(
            key,
            team.assignment_unique_value(organization_id, OrganizationLedgerRole::FinanceManager)
                .unwrap()
        );
    }
}
