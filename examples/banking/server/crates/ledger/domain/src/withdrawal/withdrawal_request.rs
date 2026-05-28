use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;
use crate::payout_destination::PayoutDestinationId;

/// Describes a withdrawal request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WithdrawalRequest {
    pub account_id: AccountId,
    pub currency_id: CurrencyId,
    pub payout_destination_id: PayoutDestinationId,
    pub amount: CurrencyAmount,
}

impl WithdrawalRequest {
    /// Returns the withdrawal amount.
    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub(super) fn into_parts(self) -> (AccountId, CurrencyId, PayoutDestinationId, CurrencyAmount) {
        (
            self.account_id,
            self.currency_id,
            self.payout_destination_id,
            self.amount,
        )
    }
}
