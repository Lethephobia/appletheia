use super::{TeamRole, TeamRoleAssignee, TeamRoleAssignmentEventPayloadError};
use crate::TeamId;
use appletheia::event_payload;

#[event_payload(error = TeamRoleAssignmentEventPayloadError)]
pub enum TeamRoleAssignmentEventPayload {
    Assigned {
        team_id: TeamId,
        assignee: TeamRoleAssignee,
        role: TeamRole,
    },
    Revoked,
}
