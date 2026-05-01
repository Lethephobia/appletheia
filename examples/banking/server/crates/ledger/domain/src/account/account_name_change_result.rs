use super::AccountNameChangeRejectionReason;

/// Describes the domain outcome of an account name change request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountNameChangeResult {
    Changed,
    Rejected {
        reason: AccountNameChangeRejectionReason,
    },
}
