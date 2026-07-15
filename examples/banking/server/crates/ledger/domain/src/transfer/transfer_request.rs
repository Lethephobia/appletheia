use crate::account::AccountId;
use crate::core::CurrencyAmount;

/// Describes a transfer request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TransferRequest {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
}

impl TransferRequest {
    /// Returns whether the transfer source and destination are the same account.
    pub fn is_same_account(&self) -> bool {
        self.from_account_id == self.to_account_id
    }

    /// Returns the transfer amount.
    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub(super) fn into_parts(self) -> (AccountId, AccountId, CurrencyAmount) {
        (self.from_account_id, self.to_account_id, self.amount)
    }
}
