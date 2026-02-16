use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CaveatNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CaveatName(String);

impl CaveatName {
    pub const MAX_LENGTH: usize = 50;

    pub fn new(value: String) -> Result<Self, CaveatNameError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), CaveatNameError> {
        if value.is_empty() {
            return Err(CaveatNameError::Empty);
        }
        let len = value.len();
        if len > Self::MAX_LENGTH {
            return Err(CaveatNameError::TooLong {
                len,
                max: Self::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(CaveatNameError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl FromStr for CaveatName {
    type Err = CaveatNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned())
    }
}

impl TryFrom<&str> for CaveatName {
    type Error = CaveatNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CaveatName {
    type Error = CaveatNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for CaveatName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        let err = CaveatName::try_from("").expect_err("empty should be rejected");
        assert!(matches!(err, CaveatNameError::Empty));
    }

    #[test]
    fn rejects_invalid_format() {
        let err = CaveatName::try_from("BizHours").expect_err("invalid should be rejected");
        assert!(matches!(err, CaveatNameError::InvalidFormat { .. }));
    }

    #[test]
    fn rejects_too_long() {
        let long = "a".repeat(CaveatName::MAX_LENGTH + 1);
        let err = CaveatName::try_from(long).expect_err("too long should be rejected");
        assert!(matches!(err, CaveatNameError::TooLong { .. }));
    }

    #[test]
    fn accepts_snake_case() {
        let name = CaveatName::try_from("business_hours").expect("valid");
        assert_eq!(name.value(), "business_hours");
    }
}

