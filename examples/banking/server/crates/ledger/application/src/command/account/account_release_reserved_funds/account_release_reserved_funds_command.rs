use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Releases reserved funds in the specified account.
#[command(name = "account_release_reserved_funds")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReleaseReservedFundsCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
