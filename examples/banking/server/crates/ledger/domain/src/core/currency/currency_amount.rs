use std::fmt::{self, Display};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::CurrencyAmountError;

/// Represents a balance amount in the smallest unit of an account currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurrencyAmount(u128);

impl CurrencyAmount {
    /// Creates an amount from the smallest-unit quantity.
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns a zero amount.
    pub fn zero() -> Self {
        Self(0)
    }

    /// Returns the smallest-unit amount.
    pub fn value(&self) -> u128 {
        self.0
    }

    /// Returns whether the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Adds another amount and returns the resulting amount.
    pub fn try_add(self, amount: Self) -> Result<Self, CurrencyAmountError> {
        let value = self
            .value()
            .checked_add(amount.value())
            .ok_or(CurrencyAmountError::BalanceOverflow)?;

        Ok(Self::new(value))
    }

    /// Subtracts another amount and returns the resulting amount.
    pub fn try_sub(self, amount: Self) -> Result<Self, CurrencyAmountError> {
        let value = self
            .value()
            .checked_sub(amount.value())
            .ok_or(CurrencyAmountError::InsufficientBalance)?;

        Ok(Self::new(value))
    }
}

impl From<u128> for CurrencyAmount {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl Serialize for CurrencyAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value().to_string())
    }
}

impl<'de> Deserialize<'de> for CurrencyAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.parse::<u128>().map_err(serde::de::Error::custom)?;

        Ok(CurrencyAmount::new(value))
    }
}

impl Display for CurrencyAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyAmount;
    use crate::core::CurrencyAmountError;

    #[test]
    fn zero_returns_zero_amount() {
        assert_eq!(CurrencyAmount::zero().value(), 0);
    }

    #[test]
    fn try_add_returns_added_amount() {
        let amount = CurrencyAmount::new(10)
            .try_add(CurrencyAmount::new(5))
            .expect("addition should succeed");

        assert_eq!(amount, CurrencyAmount::new(15));
    }

    #[test]
    fn try_add_returns_overflow_error() {
        let error = CurrencyAmount::new(u128::MAX)
            .try_add(CurrencyAmount::new(1))
            .expect_err("overflow should fail");

        assert!(matches!(error, CurrencyAmountError::BalanceOverflow));
    }

    #[test]
    fn try_sub_returns_subtracted_amount() {
        let amount = CurrencyAmount::new(10)
            .try_sub(CurrencyAmount::new(5))
            .expect("subtraction should succeed");

        assert_eq!(amount, CurrencyAmount::new(5));
    }

    #[test]
    fn try_sub_returns_insufficient_balance_error() {
        let error = CurrencyAmount::new(1)
            .try_sub(CurrencyAmount::new(2))
            .expect_err("subtraction should fail");

        assert!(matches!(error, CurrencyAmountError::InsufficientBalance));
    }

    #[test]
    fn serializes_to_json_string() {
        let value =
            serde_json::to_value(CurrencyAmount::new(42)).expect("serialize should succeed");

        assert_eq!(value, serde_json::Value::String("42".to_owned()));
    }

    #[test]
    fn deserializes_from_json_string() {
        let amount =
            serde_json::from_value::<CurrencyAmount>(serde_json::Value::String("42".to_owned()))
                .expect("deserialize should succeed");

        assert_eq!(amount, CurrencyAmount::new(42));
    }

    #[test]
    fn rejects_json_integer() {
        let error =
            serde_json::from_value::<CurrencyAmount>(serde_json::json!(42)).expect_err("reject");

        assert!(error.is_data());
    }
}
