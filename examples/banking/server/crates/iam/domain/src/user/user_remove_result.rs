use super::UserStatusRejectionReason;

/// Describes the domain outcome of a user removal operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserRemoveResult {
    Removed,
    Rejected { reason: UserStatusRejectionReason },
}
