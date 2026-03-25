use banking_iam_domain::RoleId;
use serde::{Deserialize, Serialize};

/// The output returned after creating a role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleCreateOutput {
    pub role_id: RoleId,
}

impl RoleCreateOutput {
    /// Creates a new role-create output.
    pub fn new(role_id: RoleId) -> Self {
        Self { role_id }
    }
}
