use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

/// Requests a transfer between accounts.
#[command(name = "transfer_request")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequestCommand {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: AccountBalance,
}
