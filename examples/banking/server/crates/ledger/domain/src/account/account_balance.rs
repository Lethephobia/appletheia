use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::AccountStateError;

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

    /// Adds another balance and returns the resulting amount.
    pub fn try_add(self, amount: Self) -> Result<Self, AccountStateError> {
        let value = self
            .value()
            .checked_add(amount.value())
            .ok_or(AccountStateError::BalanceOverflow)?;

        Ok(Self::new(value))
    }

    /// Subtracts another balance and returns the resulting amount.
    pub fn try_sub(self, amount: Self) -> Result<Self, AccountStateError> {
        let value = self
            .value()
            .checked_sub(amount.value())
            .ok_or(AccountStateError::InsufficientBalance)?;

        Ok(Self::new(value))
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
    use crate::account::AccountStateError;

    #[test]
    fn zero_returns_zero_balance() {
        assert_eq!(AccountBalance::zero().value(), 0);
    }

    #[test]
    fn try_add_returns_added_balance() {
        let balance = AccountBalance::new(10)
            .try_add(AccountBalance::new(5))
            .expect("addition should succeed");

        assert_eq!(balance, AccountBalance::new(15));
    }

    #[test]
    fn try_add_returns_overflow_error() {
        let error = AccountBalance::new(u128::MAX)
            .try_add(AccountBalance::new(1))
            .expect_err("overflow should fail");

        assert!(matches!(error, AccountStateError::BalanceOverflow));
    }

    #[test]
    fn try_sub_returns_subtracted_balance() {
        let balance = AccountBalance::new(10)
            .try_sub(AccountBalance::new(5))
            .expect("subtraction should succeed");

        assert_eq!(balance, AccountBalance::new(5));
    }

    #[test]
    fn try_sub_returns_insufficient_balance_error() {
        let error = AccountBalance::new(1)
            .try_sub(AccountBalance::new(2))
            .expect_err("subtraction should fail");

        assert!(matches!(error, AccountStateError::InsufficientBalance));
    }
}
