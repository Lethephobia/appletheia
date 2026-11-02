use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Reserves funds in the specified account.
#[command(name = "account_funds_reserve")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountFundsReserveCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
