use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Commits reserved funds in the specified account.
#[command(name = "account_reserved_funds_commit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReservedFundsCommitCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
