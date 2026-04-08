use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use serde::{Deserialize, Serialize};

/// Deposits into the specified account.
#[command(name = "account_deposit")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountDepositCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
