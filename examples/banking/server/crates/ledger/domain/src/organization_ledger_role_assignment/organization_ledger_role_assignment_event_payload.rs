use super::{
    OrganizationLedgerRole, OrganizationLedgerRoleAssignee,
    OrganizationLedgerRoleAssignmentEventPayloadError,
};
use appletheia::event_payload;
use banking_iam_domain::OrganizationId;

#[event_payload(error = OrganizationLedgerRoleAssignmentEventPayloadError)]
pub enum OrganizationLedgerRoleAssignmentEventPayload {
    Assigned {
        organization_id: OrganizationId,
        assignee: OrganizationLedgerRoleAssignee,
        role: OrganizationLedgerRole,
    },
    Revoked,
}
