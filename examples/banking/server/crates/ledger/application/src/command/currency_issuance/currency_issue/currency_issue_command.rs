use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Starts a currency issuance workflow.
#[command(name = "currency_issue")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyIssueCommand {
    pub currency_id: CurrencyId,
    pub destination_account_id: AccountId,
    pub amount: CurrencyAmount,
}
