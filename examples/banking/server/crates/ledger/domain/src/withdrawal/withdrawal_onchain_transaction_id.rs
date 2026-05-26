use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Represents an on-chain transaction identifier for a withdrawal.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WithdrawalOnchainTransactionId(String);

impl WithdrawalOnchainTransactionId {
    /// Creates a withdrawal on-chain transaction id.
    pub fn new(value: String) -> Option<Self> {
        let value = value.trim().to_owned();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return None;
        }
        Some(Self(value))
    }

    /// Returns the on-chain transaction id value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WithdrawalOnchainTransactionId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for WithdrawalOnchainTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for WithdrawalOnchainTransactionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned()).ok_or(())
    }
}

impl TryFrom<&str> for WithdrawalOnchainTransactionId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for WithdrawalOnchainTransactionId {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl From<WithdrawalOnchainTransactionId> for String {
    fn from(value: WithdrawalOnchainTransactionId) -> Self {
        value.0
    }
}
