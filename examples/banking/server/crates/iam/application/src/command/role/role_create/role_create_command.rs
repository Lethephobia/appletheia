use appletheia::application::command::{Command, CommandName};
use banking_iam_domain::RoleName;
use serde::{Deserialize, Serialize};

/// Creates a new role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleCreateCommand {
    pub name: RoleName,
}

impl Command for RoleCreateCommand {
    const NAME: CommandName = CommandName::new("role_create");
}
