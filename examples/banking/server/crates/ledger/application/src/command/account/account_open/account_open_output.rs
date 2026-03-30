use banking_ledger_domain::account::AccountId;
use serde::{Deserialize, Serialize};

/// The output returned after opening an account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOpenOutput {
    pub account_id: AccountId,
}

impl AccountOpenOutput {
    /// Creates a new account-open output.
    pub fn new(account_id: AccountId) -> Self {
        Self { account_id }
    }
}
