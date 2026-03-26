use appletheia::command;
use banking_iam_domain::UserRoleAssignmentId;
use serde::{Deserialize, Serialize};

/// Revokes a role assignment from a user.
#[command(name = "user_role_assignment_revoke")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentRevokeCommand {
    pub user_role_assignment_id: UserRoleAssignmentId,
}
