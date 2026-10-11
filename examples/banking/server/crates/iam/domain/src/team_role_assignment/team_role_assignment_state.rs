use super::{TeamRole, TeamRoleAssignee, TeamRoleAssignmentStateError, TeamRoleAssignmentStatus};
use crate::TeamId;
use appletheia::domain::UniqueValue;
use appletheia::{aggregate_state, reference_indexes, unique_constraints};
use uuid::Uuid;

#[aggregate_state(error = TeamRoleAssignmentStateError)]
#[unique_constraints(entry(key = "team_assignee_role", value = assignment_unique_value))]
#[reference_indexes(entry(key = "team", value = team_ref_value))]
pub struct TeamRoleAssignmentState {
    pub(super) team_id: TeamId,
    pub(super) assignee: TeamRoleAssignee,
    pub(super) role: TeamRole,
    pub(super) status: TeamRoleAssignmentStatus,
}

fn assignment_unique_value(
    state: &TeamRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, TeamRoleAssignmentStateError> {
    if state.status != TeamRoleAssignmentStatus::Active {
        return Ok(None);
    }
    Ok(Some(
        state
            .assignee
            .assignment_unique_value(state.team_id, state.role)?,
    ))
}

fn team_ref_value(
    state: &TeamRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<TeamId>, TeamRoleAssignmentStateError> {
    Ok(Some(state.team_id))
}
