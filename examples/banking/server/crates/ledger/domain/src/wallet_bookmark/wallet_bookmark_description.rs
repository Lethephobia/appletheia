use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::WalletBookmarkDescriptionError;

/// Represents a user-facing wallet bookmark description.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WalletBookmarkDescription(String);

impl WalletBookmarkDescription {
    /// Creates a wallet bookmark description from user input.
    pub fn new(value: String) -> Result<Self, WalletBookmarkDescriptionError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(WalletBookmarkDescriptionError::Empty);
        }

        if normalized.chars().count() > 280 {
            return Err(WalletBookmarkDescriptionError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated description.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WalletBookmarkDescription {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for WalletBookmarkDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for WalletBookmarkDescription {
    type Err = WalletBookmarkDescriptionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for WalletBookmarkDescription {
    type Error = WalletBookmarkDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for WalletBookmarkDescription {
    type Error = WalletBookmarkDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<WalletBookmarkDescription> for String {
    fn from(value: WalletBookmarkDescription) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{WalletBookmarkDescription, WalletBookmarkDescriptionError};

    #[test]
    fn accepts_valid_description() {
        let description = WalletBookmarkDescription::try_from("  Personal main wallet  ")
            .expect("description should be valid");

        assert_eq!(description.value(), "Personal main wallet");
    }

    #[test]
    fn rejects_empty_description() {
        let error =
            WalletBookmarkDescription::try_from("   ").expect_err("empty description should fail");

        assert!(matches!(error, WalletBookmarkDescriptionError::Empty));
    }

    #[test]
    fn rejects_too_long_description() {
        let value = "a".repeat(281);
        let error =
            WalletBookmarkDescription::try_from(value).expect_err("description should be too long");

        assert!(matches!(error, WalletBookmarkDescriptionError::TooLong));
    }
}
