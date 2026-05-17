use super::CurrencyMintAccountRecordRejectionReason;

/// Represents whether mint account recording can start.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurrencyMintAccountRecordPreparationResult {
    Ready,
    Rejected {
        reason: CurrencyMintAccountRecordRejectionReason,
    },
}
