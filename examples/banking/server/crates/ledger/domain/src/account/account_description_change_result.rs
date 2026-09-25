use super::AccountDescriptionChangeRejectionReason;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountDescriptionChangeResult {
    Changed,
    Rejected {
        reason: AccountDescriptionChangeRejectionReason,
    },
}
