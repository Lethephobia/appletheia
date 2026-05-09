use super::UserPictureChangeRejectionReason;

/// Describes the domain outcome of a user picture change operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserPictureChangeResult {
    Changed,
    Rejected {
        reason: UserPictureChangeRejectionReason,
    },
}
