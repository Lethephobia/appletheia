use super::UserStatusRejectionReason;

/// Describes the domain outcome of a user activation operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserActivateResult {
    Activated,
    Rejected { reason: UserStatusRejectionReason },
}
