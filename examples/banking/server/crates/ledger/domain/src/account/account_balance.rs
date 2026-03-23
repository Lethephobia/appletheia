use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Represents a balance amount in the smallest unit of an account currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountBalance(u128);

impl AccountBalance {
    /// Creates a balance from the smallest-unit amount.
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns a zero balance.
    pub fn zero() -> Self {
        Self(0)
    }

    /// Returns the smallest-unit amount.
    pub fn value(&self) -> u128 {
        self.0
    }

    /// Returns whether the balance is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<u128> for AccountBalance {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl Display for AccountBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::AccountBalance;

    #[test]
    fn zero_returns_zero_balance() {
        assert_eq!(AccountBalance::zero().value(), 0);
    }
}
