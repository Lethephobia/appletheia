use appletheia::command;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use serde::{Deserialize, Serialize};

/// Transfers ownership of an account.
#[command(name = "account_transfer_ownership")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountTransferOwnershipCommand {
    pub account_id: AccountId,
    pub owner: AccountOwner,
}
