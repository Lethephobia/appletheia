use super::UserBioChangeRejectionReason;

/// Describes the domain outcome of a user bio change operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserBioChangeResult {
    Changed,
    Rejected {
        reason: UserBioChangeRejectionReason,
    },
}
