use super::UserDisplayNameChangeRejectionReason;

/// Describes the domain outcome of a user display name change operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserDisplayNameChangeResult {
    Changed,
    Rejected {
        reason: UserDisplayNameChangeRejectionReason,
    },
}
