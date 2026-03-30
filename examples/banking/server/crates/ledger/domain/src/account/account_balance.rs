use std::fmt::{self, Display};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::AccountBalanceError;

/// Represents a balance amount in the smallest unit of an account currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    pub fn try_add(self, amount: Self) -> Result<Self, AccountBalanceError> {
        let value = self
            .value()
            .checked_add(amount.value())
            .ok_or(AccountBalanceError::BalanceOverflow)?;

        Ok(Self::new(value))
    }

    /// Subtracts another balance and returns the resulting amount.
    pub fn try_sub(self, amount: Self) -> Result<Self, AccountBalanceError> {
        let value = self
            .value()
            .checked_sub(amount.value())
            .ok_or(AccountBalanceError::InsufficientBalance)?;

        Ok(Self::new(value))
    }
}

impl From<u128> for AccountBalance {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl Serialize for AccountBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value().to_string())
    }
}

impl<'de> Deserialize<'de> for AccountBalance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.parse::<u128>().map_err(serde::de::Error::custom)?;

        Ok(AccountBalance::new(value))
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
    use crate::account::AccountBalanceError;

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

        assert!(matches!(error, AccountBalanceError::BalanceOverflow));
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

        assert!(matches!(error, AccountBalanceError::InsufficientBalance));
    }

    #[test]
    fn serializes_to_json_string() {
        let value =
            serde_json::to_value(AccountBalance::new(42)).expect("serialize should succeed");

        assert_eq!(value, serde_json::Value::String("42".to_owned()));
    }

    #[test]
    fn deserializes_from_json_string() {
        let balance =
            serde_json::from_value::<AccountBalance>(serde_json::Value::String("42".to_owned()))
                .expect("deserialize should succeed");

        assert_eq!(balance, AccountBalance::new(42));
    }

    #[test]
    fn rejects_json_integer() {
        let error =
            serde_json::from_value::<AccountBalance>(serde_json::json!(42)).expect_err("reject");

        assert!(error.is_data());
    }
}
