use super::DepositTokenTransferRecordRejectionReason;

/// Describes the domain outcome of recording an on-chain token transfer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositTokenTransferResult {
    TokenTransferred,
    Rejected {
        reason: DepositTokenTransferRecordRejectionReason,
    },
}
