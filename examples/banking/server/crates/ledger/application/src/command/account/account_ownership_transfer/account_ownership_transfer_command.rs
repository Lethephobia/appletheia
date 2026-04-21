use appletheia::command;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use serde::{Deserialize, Serialize};

/// Transfers ownership of an account.
#[command(name = "account_ownership_transfer")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOwnershipTransferCommand {
    pub account_id: AccountId,
    pub owner: AccountOwner,
}
