use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

use super::AccountReleaseReservedFundsContext;

/// Releases reserved funds in the specified account.
#[command(name = "account_release_reserved_funds")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReleaseReservedFundsCommand {
    pub account_id: AccountId,
    pub amount: AccountBalance,
    #[serde(default)]
    pub context: AccountReleaseReservedFundsContext,
}
