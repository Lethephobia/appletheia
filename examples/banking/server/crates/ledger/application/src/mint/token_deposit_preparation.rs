use serde::{Deserialize, Serialize};

use super::PreparedTokenDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenDepositPreparation {
    pub transaction: PreparedTokenDepositTransaction,
}

impl TokenDepositPreparation {
    pub fn new(transaction: PreparedTokenDepositTransaction) -> Self {
        Self { transaction }
    }
}
