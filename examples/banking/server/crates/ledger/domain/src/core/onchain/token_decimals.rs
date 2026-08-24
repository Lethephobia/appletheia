use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Represents the decimal precision of an onchain token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TokenDecimals(u8);

impl TokenDecimals {
    /// Creates token decimals from an integer precision.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the decimal precision.
    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for TokenDecimals {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl Display for TokenDecimals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
