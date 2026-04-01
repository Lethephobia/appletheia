use banking_ledger_domain::account::{AccountId, AccountName};
use serde::{Deserialize, Serialize};

/// The output returned after renaming an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRenameOutput {
    pub account_id: AccountId,
    pub name: AccountName,
}

impl AccountRenameOutput {
    /// Creates a new account-rename output.
    pub fn new(account_id: AccountId, name: AccountName) -> Self {
        Self { account_id, name }
    }
}
