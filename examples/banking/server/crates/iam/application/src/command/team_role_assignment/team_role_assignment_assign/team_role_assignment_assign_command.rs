use appletheia::command;
use banking_iam_domain::TeamId;
use banking_iam_domain::{TeamRole, TeamRoleAssignee};
use serde::{Deserialize, Serialize};

#[command(name = "team_role_assignment_assign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamRoleAssignmentAssignCommand {
    pub team_id: TeamId,
    pub assignee: TeamRoleAssignee,
    pub role: TeamRole,
}
