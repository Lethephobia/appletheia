use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Represents an amount in on-chain token base units.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TokenAmount(u128);

impl TokenAmount {
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u128 {
        self.0
    }
}

impl From<u128> for TokenAmount {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<TokenAmount> for u128 {
    fn from(value: TokenAmount) -> Self {
        value.value()
    }
}

impl Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
