use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::WalletBookmarkDisplayNameError;

/// Represents a validated wallet bookmark display name.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WalletBookmarkDisplayName(String);

impl WalletBookmarkDisplayName {
    /// Creates a wallet bookmark display name from user input.
    pub fn new(value: String) -> Result<Self, WalletBookmarkDisplayNameError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(WalletBookmarkDisplayNameError::Empty);
        }

        if normalized.len() > 100 {
            return Err(WalletBookmarkDisplayNameError::TooLong);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated display name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WalletBookmarkDisplayName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for WalletBookmarkDisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for WalletBookmarkDisplayName {
    type Err = WalletBookmarkDisplayNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for WalletBookmarkDisplayName {
    type Error = WalletBookmarkDisplayNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for WalletBookmarkDisplayName {
    type Error = WalletBookmarkDisplayNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<WalletBookmarkDisplayName> for String {
    fn from(value: WalletBookmarkDisplayName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{WalletBookmarkDisplayName, WalletBookmarkDisplayNameError};

    #[test]
    fn accepts_valid_display_name() {
        let display_name = WalletBookmarkDisplayName::try_from("  Main wallet  ")
            .expect("display name should be valid");

        assert_eq!(display_name.value(), "Main wallet");
    }

    #[test]
    fn rejects_empty_display_name() {
        let error =
            WalletBookmarkDisplayName::try_from("   ").expect_err("empty display name should fail");

        assert!(matches!(error, WalletBookmarkDisplayNameError::Empty));
    }

    #[test]
    fn rejects_too_long_display_name() {
        let value = "a".repeat(101);
        let error = WalletBookmarkDisplayName::try_from(value)
            .expect_err("display name should be too long");

        assert!(matches!(error, WalletBookmarkDisplayNameError::TooLong));
    }
}
