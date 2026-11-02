use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Releases reserved funds in the specified account.
#[command(name = "account_reserved_funds_release")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReservedFundsReleaseCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
