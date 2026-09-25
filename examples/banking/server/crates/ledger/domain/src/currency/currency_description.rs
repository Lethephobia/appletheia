use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyDescriptionError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CurrencyDescription(String);

impl CurrencyDescription {
    pub fn new(value: String) -> Result<Self, CurrencyDescriptionError> {
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(CurrencyDescriptionError::Empty);
        }
        if normalized.chars().count() > 280 {
            return Err(CurrencyDescriptionError::TooLong);
        }
        Ok(Self(normalized.to_owned()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

impl FromStr for CurrencyDescription {
    type Err = CurrencyDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyDescription {
    type Error = CurrencyDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyDescription {
    type Error = CurrencyDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyDescription> for String {
    fn from(value: CurrencyDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyDescription, CurrencyDescriptionError};

    #[test]
    fn validates_and_normalizes_currency_descriptions() {
        let description = CurrencyDescription::try_from("  United States dollar  ")
            .expect("description should be valid");
        assert_eq!(description.value(), "United States dollar");
        assert_eq!(
            CurrencyDescription::try_from(" ").expect_err("empty description should fail"),
            CurrencyDescriptionError::Empty
        );
    }
}
