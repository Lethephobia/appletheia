use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

/// Describes a withdrawal request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithdrawalRequest {
    pub account_id: AccountId,
    pub currency_id: CurrencyId,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub amount: CurrencyAmount,
}

impl WithdrawalRequest {
    /// Returns the withdrawal amount.
    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        AccountId,
        CurrencyId,
        TokenAccountOwnerAddress,
        CurrencyAmount,
    ) {
        (
            self.account_id,
            self.currency_id,
            self.token_account_owner_address,
            self.amount,
        )
    }
}
