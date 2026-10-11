use super::{
    OrganizationLedgerRole, OrganizationLedgerRoleAssignee,
    OrganizationLedgerRoleAssignmentStateError, OrganizationLedgerRoleAssignmentStatus,
};
use appletheia::domain::UniqueValue;
use appletheia::{aggregate_state, reference_indexes, unique_constraints};
use banking_iam_domain::OrganizationId;
use uuid::Uuid;

#[aggregate_state(error = OrganizationLedgerRoleAssignmentStateError)]
#[unique_constraints(entry(key = "organization_assignee_role", value = assignment_unique_value))]
#[reference_indexes(entry(key = "organization", value = organization_ref_value))]
pub struct OrganizationLedgerRoleAssignmentState {
    pub(super) organization_id: OrganizationId,
    pub(super) assignee: OrganizationLedgerRoleAssignee,
    pub(super) role: OrganizationLedgerRole,
    pub(super) status: OrganizationLedgerRoleAssignmentStatus,
}

fn assignment_unique_value(
    state: &OrganizationLedgerRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, OrganizationLedgerRoleAssignmentStateError> {
    if state.status != OrganizationLedgerRoleAssignmentStatus::Active {
        return Ok(None);
    }
    Ok(Some(state.assignee.assignment_unique_value(
        state.organization_id,
        state.role,
    )?))
}

fn organization_ref_value(
    state: &OrganizationLedgerRoleAssignmentState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, OrganizationLedgerRoleAssignmentStateError> {
    Ok(Some(state.organization_id))
}
