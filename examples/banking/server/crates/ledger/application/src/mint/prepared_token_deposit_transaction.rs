use serde::{Deserialize, Serialize};

/// Contains a token-deposit transaction prepared for client signing and submission.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PreparedTokenDepositTransaction(String);

impl PreparedTokenDepositTransaction {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
