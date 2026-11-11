use std::fmt::{self, Display};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a non-negative token quantity in token base units.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TokenAmount(u128);

impl TokenAmount {
    /// Creates a token amount expressed in base units.
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the underlying base-unit quantity.
    pub const fn value(&self) -> u128 {
        self.0
    }
}

impl From<u128> for TokenAmount {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl Serialize for TokenAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for TokenAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let parsed_value = value.parse::<u128>().map_err(serde::de::Error::custom)?;
        Ok(Self::new(parsed_value))
    }
}

impl Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::TokenAmount;

    #[test]
    fn serializes_as_a_json_string_without_float_conversion() {
        let amount = TokenAmount::new(u128::MAX);
        let json = serde_json::to_value(amount).expect("serialization should succeed");

        assert_eq!(json, serde_json::Value::String(u128::MAX.to_string()));
        assert_eq!(
            serde_json::from_value::<TokenAmount>(json).expect("deserialization should succeed"),
            amount
        );
    }
}
