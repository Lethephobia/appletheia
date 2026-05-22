use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::OnchainAccountAddressError;

/// Represents an on-chain account address passed to an on-chain gateway.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OnchainAccountAddress(String);

impl OnchainAccountAddress {
    /// Creates an on-chain account address.
    pub fn new(value: String) -> Result<Self, OnchainAccountAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(OnchainAccountAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(OnchainAccountAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the account address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OnchainAccountAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OnchainAccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OnchainAccountAddress {
    type Err = OnchainAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for OnchainAccountAddress {
    type Error = OnchainAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OnchainAccountAddress {
    type Error = OnchainAccountAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OnchainAccountAddress> for String {
    fn from(value: OnchainAccountAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{OnchainAccountAddress, OnchainAccountAddressError};

    #[test]
    fn accepts_valid_account_address() {
        let address = OnchainAccountAddress::try_from("Account111111111111111111111111111111111")
            .expect("address should be valid");

        assert_eq!(address.value(), "Account111111111111111111111111111111111");
    }

    #[test]
    fn rejects_empty_account_address() {
        let error = OnchainAccountAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, OnchainAccountAddressError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_account_address() {
        let error = OnchainAccountAddress::try_from("Account 111")
            .expect_err("invalid address should fail");

        assert!(matches!(error, OnchainAccountAddressError::InvalidFormat));
    }
}
