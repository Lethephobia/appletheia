use banking_iam_domain::UserRoleAssignmentId;
use serde::{Deserialize, Serialize};

/// The output returned after assigning a role to a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoleAssignmentAssignOutput {
    pub user_role_assignment_id: UserRoleAssignmentId,
}

impl UserRoleAssignmentAssignOutput {
    /// Creates a new assignment output.
    pub fn new(user_role_assignment_id: UserRoleAssignmentId) -> Self {
        Self {
            user_role_assignment_id,
        }
    }
}
