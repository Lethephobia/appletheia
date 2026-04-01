use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

use super::AccountCommitReservedFundsContext;

/// Commits reserved funds in the specified account.
#[command(name = "account_commit_reserved_funds")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountCommitReservedFundsCommand {
    pub account_id: AccountId,
    pub amount: AccountBalance,
    pub context: AccountCommitReservedFundsContext,
}
