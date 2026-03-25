pub mod user_role_assignment_assign;
pub mod user_role_assignment_revoke;

pub use user_role_assignment_assign::{
    UserRoleAssignmentAssignCommand, UserRoleAssignmentAssignCommandHandler,
    UserRoleAssignmentAssignOutput,
};
pub use user_role_assignment_revoke::{
    UserRoleAssignmentRevokeCommand, UserRoleAssignmentRevokeCommandHandler,
    UserRoleAssignmentRevokeOutput,
};
