use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::payout_destination::PayoutDestinationId;
use serde::{Deserialize, Serialize};

/// Requests a withdrawal from an account to a payout destination.
#[command(name = "withdrawal_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRequestCommand {
    pub account_id: AccountId,
    pub payout_destination_id: PayoutDestinationId,
    pub amount: CurrencyAmount,
}
