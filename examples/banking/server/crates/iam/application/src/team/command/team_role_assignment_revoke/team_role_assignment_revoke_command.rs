use appletheia::command;
use banking_iam_domain::TeamRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "team_role_assignment_revoke")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamRoleAssignmentRevokeCommand {
    pub team_role_assignment_id: TeamRoleAssignmentId,
}
