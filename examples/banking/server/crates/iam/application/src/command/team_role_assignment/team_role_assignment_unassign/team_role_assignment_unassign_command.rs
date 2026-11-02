use appletheia::command;
use banking_iam_domain::TeamRoleAssignmentId;
use serde::{Deserialize, Serialize};

#[command(name = "team_role_assignment_unassign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamRoleAssignmentUnassignCommand {
    pub team_role_assignment_id: TeamRoleAssignmentId,
}
