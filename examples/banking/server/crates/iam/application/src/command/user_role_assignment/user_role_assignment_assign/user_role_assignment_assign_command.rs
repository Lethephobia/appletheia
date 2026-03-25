use appletheia::application::command::{Command, CommandName};
use banking_iam_domain::{RoleId, UserId};
use serde::{Deserialize, Serialize};

/// Assigns a role to a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentAssignCommand {
    pub role_id: RoleId,
    pub user_id: UserId,
}

impl Command for UserRoleAssignmentAssignCommand {
    const NAME: CommandName = CommandName::new("user_role_assignment_assign");
}
