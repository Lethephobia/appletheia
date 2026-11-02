mod team_role;
mod team_role_assignee;
mod team_role_assignee_error;
mod team_role_assignment_error;
mod team_role_assignment_event_payload;
mod team_role_assignment_event_payload_error;
mod team_role_assignment_id;
mod team_role_assignment_state;
mod team_role_assignment_state_error;
mod team_role_assignment_status;

pub use team_role::*;
pub use team_role_assignee::*;
pub use team_role_assignee_error::*;
pub use team_role_assignment_error::*;
pub use team_role_assignment_event_payload::*;
pub use team_role_assignment_event_payload_error::*;
pub use team_role_assignment_id::*;
pub use team_role_assignment_state::*;
pub use team_role_assignment_state_error::*;
pub use team_role_assignment_status::*;

use crate::TeamId;
use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

#[aggregate(type = "team_role_assignment", error = TeamRoleAssignmentError)]
pub struct TeamRoleAssignment {
    core: AggregateCore<
        TeamRoleAssignmentId,
        TeamRoleAssignmentState,
        TeamRoleAssignmentEventPayload,
    >,
}

impl TeamRoleAssignment {
    pub fn team_id(&self) -> Result<TeamId, TeamRoleAssignmentError> {
        Ok(self.state_required()?.team_id)
    }

    pub fn assignee(&self) -> Result<TeamRoleAssignee, TeamRoleAssignmentError> {
        Ok(self.state_required()?.assignee)
    }

    pub fn role(&self) -> Result<TeamRole, TeamRoleAssignmentError> {
        Ok(self.state_required()?.role)
    }

    pub fn status(&self) -> Result<TeamRoleAssignmentStatus, TeamRoleAssignmentError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, TeamRoleAssignmentError> {
        Ok(self.status()? == TeamRoleAssignmentStatus::Active)
    }

    pub fn assign(
        &mut self,
        team_id: TeamId,
        assignee: TeamRoleAssignee,
        role: TeamRole,
    ) -> Result<(), TeamRoleAssignmentError> {
        if self.state().is_some() {
            return Err(TeamRoleAssignmentError::AlreadyAssigned);
        }
        self.append_event(TeamRoleAssignmentEventPayload::Assigned {
            team_id,
            assignee,
            role,
        })?;
        Ok(())
    }

    pub fn unassign(&mut self) -> Result<(), TeamRoleAssignmentError> {
        if !self.is_active()? {
            return Err(TeamRoleAssignmentError::Unassigned);
        }
        self.append_event(TeamRoleAssignmentEventPayload::Unassigned)?;
        Ok(())
    }
}

impl AggregateApply<TeamRoleAssignmentEventPayload, TeamRoleAssignmentError>
    for TeamRoleAssignment
{
    fn apply(
        &mut self,
        payload: &TeamRoleAssignmentEventPayload,
    ) -> Result<(), TeamRoleAssignmentError> {
        match payload {
            TeamRoleAssignmentEventPayload::Assigned {
                team_id,
                assignee,
                role,
            } => self.set_state(Some(TeamRoleAssignmentState {
                team_id: *team_id,
                assignee: *assignee,
                role: *role,
                status: TeamRoleAssignmentStatus::Active,
            })),
            TeamRoleAssignmentEventPayload::Unassigned => {
                self.state_required_mut()?.status = TeamRoleAssignmentStatus::Unassigned
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TeamMembershipId;

    #[test]
    fn assignment_lifecycle_preserves_history_and_releases_uniqueness() {
        let scope_id = TeamId::new();
        let assignee = TeamRoleAssignee::TeamMembership(TeamMembershipId::new());
        let mut assignment = TeamRoleAssignment::new();
        assignment
            .assign(scope_id, assignee, TeamRole::Manager)
            .unwrap();
        assert!(!assignment.unique_entries().unwrap().is_empty());
        assert!(matches!(
            assignment.assign(scope_id, assignee, TeamRole::Manager),
            Err(TeamRoleAssignmentError::AlreadyAssigned)
        ));
        assignment.unassign().unwrap();
        assert!(assignment.unique_entries().unwrap().is_empty());
        assert!(matches!(
            assignment.unassign(),
            Err(TeamRoleAssignmentError::Unassigned)
        ));
        assert!(matches!(
            assignment.assign(scope_id, assignee, TeamRole::Manager),
            Err(TeamRoleAssignmentError::AlreadyAssigned)
        ));
        let mut restored = TeamRoleAssignment::new();
        for event in assignment.uncommitted_events() {
            restored.apply(event.payload()).unwrap();
        }
        assert_eq!(restored.state(), assignment.state());
        assert_ne!(
            assignee
                .assignment_unique_value(scope_id, TeamRole::Manager)
                .unwrap(),
            assignee
                .assignment_unique_value(TeamId::new(), TeamRole::Manager)
                .unwrap()
        );
    }
}
