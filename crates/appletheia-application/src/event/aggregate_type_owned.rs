use std::{fmt, fmt::Display, str::FromStr};

use appletheia_domain::aggregate::AggregateType;
use serde::{Deserialize, Serialize};

use super::AggregateTypeOwnedError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AggregateTypeOwned(String);

impl AggregateTypeOwned {
    pub fn new(value: String) -> Result<Self, AggregateTypeOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), AggregateTypeOwnedError> {
        if value.is_empty() {
            return Err(AggregateTypeOwnedError::Empty);
        }
        let len = value.len();
        if len > AggregateType::MAX_LENGTH {
            return Err(AggregateTypeOwnedError::TooLong {
                len,
                max: AggregateType::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(AggregateTypeOwnedError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl FromStr for AggregateTypeOwned {
    type Err = AggregateTypeOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<&str> for AggregateTypeOwned {
    type Error = AggregateTypeOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for AggregateTypeOwned {
    type Error = AggregateTypeOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl From<AggregateType> for AggregateTypeOwned {
    fn from(value: AggregateType) -> Self {
        Self(value.value().to_owned())
    }
}

impl Display for AggregateTypeOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_snake_case() {
        let agg = AggregateTypeOwned::try_from("order_event").expect("valid snake_case");
        assert_eq!(agg.value(), "order_event");
    }

    #[test]
    fn rejects_empty() {
        let err = AggregateTypeOwned::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, AggregateTypeOwnedError::Empty));
    }

    #[test]
    fn rejects_invalid_chars() {
        let err = AggregateTypeOwned::try_from("Order-Event").expect_err("invalid chars");
        assert!(matches!(err, AggregateTypeOwnedError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(AggregateType::MAX_LENGTH + 1);
        let err = AggregateTypeOwned::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, AggregateTypeOwnedError::TooLong { .. }));
    }

    #[test]
    fn from_str_and_try_from_keep_same_behavior() {
        let via_from_str: AggregateTypeOwned = "user_profile".parse().unwrap();
        let via_try_from = AggregateTypeOwned::try_from("user_profile").unwrap();
        assert_eq!(via_from_str, via_try_from);
    }

    #[test]
    fn display_formats_value() {
        let agg = AggregateTypeOwned::try_from("user_profile").unwrap();
        assert_eq!(agg.to_string(), "user_profile");
    }
}
