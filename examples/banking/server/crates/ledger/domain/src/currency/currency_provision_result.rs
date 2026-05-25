use super::{CurrencyMintAccount, CurrencyProvisionRejectionReason};

/// Represents the outcome of provisioning a currency.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyProvisionResult {
    Provisioned {
        mint_account: CurrencyMintAccount,
    },
    Rejected {
        reason: CurrencyProvisionRejectionReason,
    },
}
