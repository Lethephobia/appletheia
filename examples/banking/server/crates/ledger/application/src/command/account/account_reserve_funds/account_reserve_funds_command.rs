use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

use super::AccountReserveFundsContext;

/// Reserves funds in the specified account.
#[command(name = "account_reserve_funds")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReserveFundsCommand {
    pub account_id: AccountId,
    pub amount: AccountBalance,
    pub context: AccountReserveFundsContext,
}
