use super::{CurrencyMintAccount, CurrencyMintAccountRecordRejectionReason};

/// Represents the outcome of recording a currency mint account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyMintAccountRecordResult {
    Recorded {
        mint_account: CurrencyMintAccount,
    },
    Rejected {
        reason: CurrencyMintAccountRecordRejectionReason,
    },
}
