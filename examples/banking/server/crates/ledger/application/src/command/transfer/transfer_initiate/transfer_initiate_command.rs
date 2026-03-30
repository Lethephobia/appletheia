use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

/// Initiates a transfer between accounts.
#[command(name = "transfer_initiate")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferInitiateCommand {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: AccountBalance,
}
