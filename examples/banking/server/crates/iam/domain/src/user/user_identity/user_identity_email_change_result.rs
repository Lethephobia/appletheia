use super::UserIdentityEmailChangeRejectionReason;

/// Describes the domain outcome of a user identity email operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserIdentityEmailChangeResult {
    Changed,
    Rejected {
        reason: UserIdentityEmailChangeRejectionReason,
    },
}
