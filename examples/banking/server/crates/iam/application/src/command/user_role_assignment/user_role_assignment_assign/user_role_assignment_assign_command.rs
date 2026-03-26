use appletheia::command;
use banking_iam_domain::{RoleId, UserId};
use serde::{Deserialize, Serialize};

/// Assigns a role to a user.
#[command(name = "user_role_assignment_assign")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentAssignCommand {
    pub role_id: RoleId,
    pub user_id: UserId,
}
