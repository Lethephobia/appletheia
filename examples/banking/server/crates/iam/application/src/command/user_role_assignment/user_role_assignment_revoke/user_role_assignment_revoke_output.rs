use banking_iam_domain::UserRoleAssignmentId;
use serde::{Deserialize, Serialize};

/// The output returned after revoking a role assignment from a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentRevokeOutput {
    pub user_role_assignment_id: UserRoleAssignmentId,
}

impl UserRoleAssignmentRevokeOutput {
    /// Creates a new revoke output.
    pub fn new(user_role_assignment_id: UserRoleAssignmentId) -> Self {
        Self {
            user_role_assignment_id,
        }
    }
}
