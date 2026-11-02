use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Withdraws from the specified account.
#[command(name = "account_withdraw")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountWithdrawCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
