use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

/// Withdraws from the specified account.
#[command(name = "account_withdraw")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountWithdrawCommand {
    pub account_id: AccountId,
    pub amount: AccountBalance,
}
