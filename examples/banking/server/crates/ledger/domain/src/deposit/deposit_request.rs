use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

/// Describes a deposit requested before its on-chain token transfer.
pub struct DepositRequest {
    pub account_id: AccountId,
    pub currency_id: CurrencyId,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub amount: CurrencyAmount,
}

impl DepositRequest {
    pub fn into_parts(
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
