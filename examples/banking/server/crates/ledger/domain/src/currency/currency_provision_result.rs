use super::{CurrencyProvisionRejectionReason, MintAccount};

/// Represents the outcome of provisioning a currency.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyProvisionResult {
    Provisioned {
        mint_account: MintAccount,
    },
    Rejected {
        reason: CurrencyProvisionRejectionReason,
    },
}
