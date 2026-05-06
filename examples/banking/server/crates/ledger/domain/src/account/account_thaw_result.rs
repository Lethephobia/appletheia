use super::AccountThawRejectionReason;

/// Describes the domain outcome of a thaw request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountThawResult {
    Thawed,
    Rejected { reason: AccountThawRejectionReason },
}
