use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Represents an on-chain transaction identifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OnchainTransactionId(String);

impl OnchainTransactionId {
    pub fn new(value: String) -> Option<Self> {
        let value = value.trim().to_owned();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return None;
        }
        Some(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OnchainTransactionId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OnchainTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OnchainTransactionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned()).ok_or(())
    }
}

impl TryFrom<&str> for OnchainTransactionId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OnchainTransactionId {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl From<OnchainTransactionId> for String {
    fn from(value: OnchainTransactionId) -> Self {
        value.0
    }
}
