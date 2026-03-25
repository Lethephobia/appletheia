use appletheia::application::command::{Command, CommandName};
use banking_iam_domain::UserRoleAssignmentId;
use serde::{Deserialize, Serialize};

/// Revokes a role assignment from a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentRevokeCommand {
    pub user_role_assignment_id: UserRoleAssignmentId,
}

impl Command for UserRoleAssignmentRevokeCommand {
    const NAME: CommandName = CommandName::new("user_role_assignment_revoke");
}
