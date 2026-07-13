use super::DepositFailRejectionReason;

/// Describes the domain outcome of failing a deposit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositFailResult {
    Failed,
    Rejected { reason: DepositFailRejectionReason },
}
