use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

use super::AccountDepositContext;

/// Deposits into the specified account.
#[command(name = "account_deposit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountDepositCommand {
    pub account_id: AccountId,
    pub amount: AccountBalance,
    #[serde(default)]
    pub context: AccountDepositContext,
}
