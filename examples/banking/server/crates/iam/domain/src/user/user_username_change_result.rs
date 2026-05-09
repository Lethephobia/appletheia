use super::UserUsernameChangeRejectionReason;

/// Describes the domain outcome of a user username change operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserUsernameChangeResult {
    Changed,
    Rejected {
        reason: UserUsernameChangeRejectionReason,
    },
}
