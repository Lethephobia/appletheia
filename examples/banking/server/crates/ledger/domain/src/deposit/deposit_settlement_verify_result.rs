use super::DepositSettlementVerifyRejectionReason;

/// Describes the outcome of recording a verified deposit settlement.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositSettlementVerifyResult {
    Verified,
    Rejected {
        reason: DepositSettlementVerifyRejectionReason,
    },
}
