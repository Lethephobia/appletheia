use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::token_binding::TokenBindingId;

use super::DepositNote;

/// Describes a deposit requested before its on-chain token settlement.
pub struct DepositRequest {
    pub account_id: AccountId,
    pub token_binding_id: TokenBindingId,
    pub amount: CurrencyAmount,
    pub note: Option<DepositNote>,
}

impl DepositRequest {
    pub fn into_parts(
        self,
    ) -> (
        AccountId,
        TokenBindingId,
        CurrencyAmount,
        Option<DepositNote>,
    ) {
        (
            self.account_id,
            self.token_binding_id,
            self.amount,
            self.note,
        )
    }
}
