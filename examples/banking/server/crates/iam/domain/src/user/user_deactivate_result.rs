use super::UserStatusRejectionReason;

/// Describes the domain outcome of a user deactivation operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserDeactivateResult {
    Deactivated,
    Rejected { reason: UserStatusRejectionReason },
}
