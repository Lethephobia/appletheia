use super::UserIdentityLinkRejectionReason;

/// Describes the domain outcome of a user identity link operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserIdentityLinkResult {
    Linked,
    Rejected {
        reason: UserIdentityLinkRejectionReason,
    },
}
