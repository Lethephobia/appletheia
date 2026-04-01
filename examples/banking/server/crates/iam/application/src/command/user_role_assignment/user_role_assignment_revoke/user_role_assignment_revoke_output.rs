use serde::{Deserialize, Serialize};

/// The output returned after revoking a role assignment from a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentRevokeOutput;
