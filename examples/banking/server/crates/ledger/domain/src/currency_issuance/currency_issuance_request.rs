use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

/// Describes a currency issuance request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CurrencyIssuanceRequest {
    pub currency_id: CurrencyId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
}

impl CurrencyIssuanceRequest {
    /// Returns the requested issuance amount.
    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub(super) fn into_parts(self) -> (CurrencyId, AccountId, CurrencyAmount) {
        (self.currency_id, self.destination_account_id, self.amount)
    }
}
