use appletheia::command;
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferNote;
use serde::{Deserialize, Serialize};

/// Requests a transfer between accounts.
#[command(name = "transfer_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequestCommand {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub note: Option<TransferNote>,
}
