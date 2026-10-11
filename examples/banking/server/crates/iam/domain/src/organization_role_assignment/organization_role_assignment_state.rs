use super::{
    OrganizationRole, OrganizationRoleAssignee, OrganizationRoleAssignmentStateError,
    OrganizationRoleAssignmentStatus,
};
use crate::OrganizationId;
use appletheia::domain::UniqueValue;
use appletheia::{aggregate_state, reference_indexes, unique_constraints};
use uuid::Uuid;

#[aggregate_state(error = OrganizationRoleAssignmentStateError)]
#[unique_constraints(entry(key = "organization_assignee_role", value = assignment_unique_value))]
#[reference_indexes(entry(key = "organization", value = organization_ref_value))]
pub struct OrganizationRoleAssignmentState {
    pub(super) organization_id: OrganizationId,
    pub(super) assignee: OrganizationRoleAssignee,
    pub(super) role: OrganizationRole,
    pub(super) status: OrganizationRoleAssignmentStatus,
}

fn assignment_unique_value(
    state: &OrganizationRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, OrganizationRoleAssignmentStateError> {
    if state.status != OrganizationRoleAssignmentStatus::Active {
        return Ok(None);
    }
    Ok(Some(state.assignee.assignment_unique_value(
        state.organization_id,
        state.role,
    )?))
}

fn organization_ref_value(
    state: &OrganizationRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, OrganizationRoleAssignmentStateError> {
    Ok(Some(state.organization_id))
}
