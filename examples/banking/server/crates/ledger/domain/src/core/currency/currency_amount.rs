use std::fmt::{self, Display};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{CurrencyAmountError, CurrencyDecimals};
use crate::core::{TokenAmount, TokenAmountConversionError, TokenDecimals};

/// Represents a non-negative quantity in a currency's smallest unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurrencyAmount(u128);

impl CurrencyAmount {
    /// Creates an amount expressed in the currency's smallest unit.
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns a zero amount.
    pub fn zero() -> Self {
        Self(0)
    }

    /// Returns the underlying smallest-unit quantity.
    pub const fn value(&self) -> u128 {
        self.0
    }

    /// Returns whether this amount is zero.
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Adds another amount using checked arithmetic.
    pub fn try_add(&self, amount: &Self) -> Result<Self, CurrencyAmountError> {
        self.0
            .checked_add(amount.0)
            .map(Self)
            .ok_or(CurrencyAmountError::Overflow)
    }

    /// Subtracts another amount using checked arithmetic.
    pub fn try_sub(&self, amount: &Self) -> Result<Self, CurrencyAmountError> {
        self.0
            .checked_sub(amount.0)
            .map(Self)
            .ok_or(CurrencyAmountError::InsufficientAmount)
    }

    /// Converts this currency amount into token base units without losing precision.
    pub fn to_token_amount(
        &self,
        currency_decimals: CurrencyDecimals,
        token_decimals: TokenDecimals,
    ) -> Result<TokenAmount, TokenAmountConversionError> {
        if self.is_zero() {
            return Ok(TokenAmount::new(0));
        }
        let currency_decimals = currency_decimals.value();
        let token_decimals = token_decimals.value();
        if token_decimals >= currency_decimals {
            let factor = 10_u128
                .checked_pow(u32::from(token_decimals - currency_decimals))
                .ok_or(TokenAmountConversionError::DecimalScaleOverflow)?;
            return self
                .0
                .checked_mul(factor)
                .map(TokenAmount::new)
                .ok_or(TokenAmountConversionError::AmountOverflow);
        }

        let factor = 10_u128
            .checked_pow(u32::from(currency_decimals - token_decimals))
            .ok_or(TokenAmountConversionError::DecimalScaleOverflow)?;
        if !self.0.is_multiple_of(factor) {
            return Err(TokenAmountConversionError::InexactAmount);
        }
        Ok(TokenAmount::new(self.0 / factor))
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
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for CurrencyAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value.parse::<u128>().map_err(serde::de::Error::custom)?;
        Ok(Self::new(value))
    }
}

impl Display for CurrencyAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyAmount;
    use crate::core::{
        CurrencyAmountError, CurrencyDecimals, TokenAmount, TokenAmountConversionError,
        TokenDecimals,
    };

    #[test]
    fn checked_arithmetic_rejects_overflow_and_insufficient_amount() {
        assert!(matches!(
            CurrencyAmount::new(u128::MAX).try_add(&CurrencyAmount::new(1)),
            Err(CurrencyAmountError::Overflow)
        ));
        assert!(matches!(
            CurrencyAmount::new(0).try_sub(&CurrencyAmount::new(1)),
            Err(CurrencyAmountError::InsufficientAmount)
        ));
    }

    #[test]
    fn serializes_as_a_json_string_without_float_conversion() {
        let amount = CurrencyAmount::new(u128::MAX);
        let json = serde_json::to_value(amount).expect("serialization should succeed");

        assert_eq!(json, serde_json::Value::String(u128::MAX.to_string()));
        assert_eq!(
            serde_json::from_value::<CurrencyAmount>(json).expect("deserialization should succeed"),
            amount
        );
    }

    #[test]
    fn converts_to_token_base_units_exactly() {
        assert_eq!(
            CurrencyAmount::new(123)
                .to_token_amount(CurrencyDecimals::new(2), TokenDecimals::new(6)),
            Ok(TokenAmount::new(1_230_000))
        );
        assert_eq!(
            CurrencyAmount::new(1_230_000)
                .to_token_amount(CurrencyDecimals::new(6), TokenDecimals::new(2)),
            Ok(TokenAmount::new(123))
        );
    }

    #[test]
    fn rejects_inexact_and_overflowing_token_conversion() {
        assert_eq!(
            CurrencyAmount::new(1).to_token_amount(CurrencyDecimals::new(6), TokenDecimals::new(2)),
            Err(TokenAmountConversionError::InexactAmount)
        );
        assert_eq!(
            CurrencyAmount::new(u128::MAX)
                .to_token_amount(CurrencyDecimals::new(0), TokenDecimals::new(18)),
            Err(TokenAmountConversionError::AmountOverflow)
        );
    }

    #[test]
    fn converts_zero_without_requiring_a_representable_decimal_factor() {
        assert_eq!(
            CurrencyAmount::zero()
                .to_token_amount(CurrencyDecimals::new(0), TokenDecimals::new(u8::MAX)),
            Ok(TokenAmount::new(0))
        );
    }
}
