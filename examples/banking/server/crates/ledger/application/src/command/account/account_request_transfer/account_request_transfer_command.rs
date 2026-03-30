use appletheia::command;
use banking_ledger_domain::account::{AccountBalance, AccountId};
use serde::{Deserialize, Serialize};

/// Requests a transfer from the specified account.
#[command(name = "account_request_transfer")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRequestTransferCommand {
    pub account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: AccountBalance,
}
