use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

use super::WithdrawalNote;

/// Describes a withdrawal request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithdrawalRequest {
    pub account_id: AccountId,
    pub token_binding_id: TokenBindingId,
    pub token_owner_address: TokenOwnerAddress,
    pub amount: CurrencyAmount,
    pub note: Option<WithdrawalNote>,
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
        TokenBindingId,
        TokenOwnerAddress,
        CurrencyAmount,
        Option<WithdrawalNote>,
    ) {
        (
            self.account_id,
            self.token_binding_id,
            self.token_owner_address,
            self.amount,
            self.note,
        )
    }
}
