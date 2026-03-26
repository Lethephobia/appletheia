use appletheia::command;
use banking_iam_domain::RoleName;
use serde::{Deserialize, Serialize};

/// Creates a new role.
#[command(name = "role_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleCreateCommand {
    pub name: RoleName,
}
