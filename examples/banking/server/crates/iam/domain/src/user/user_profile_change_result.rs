use super::UserProfileChangeRejectionReason;

/// Describes the domain outcome of a user profile operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserProfileChangeResult {
    Changed,
    Rejected {
        reason: UserProfileChangeRejectionReason,
    },
}
