use super::AccountCloseRejectionReason;

/// Describes the domain outcome of a close request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountCloseResult {
    Closed,
    Rejected { reason: AccountCloseRejectionReason },
}
