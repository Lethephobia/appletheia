use super::{
    OrganizationRole, OrganizationRoleAssignee, OrganizationRoleAssignmentEventPayloadError,
};
use crate::OrganizationId;
use appletheia::event_payload;

#[event_payload(error = OrganizationRoleAssignmentEventPayloadError)]
pub enum OrganizationRoleAssignmentEventPayload {
    Assigned {
        organization_id: OrganizationId,
        assignee: OrganizationRoleAssignee,
        role: OrganizationRole,
    },
    Unassigned,
}
