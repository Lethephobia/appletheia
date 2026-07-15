use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::PoolTokenAccountAddressError;

/// Represents the on-chain pool token account address linked to a currency.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PoolTokenAccountAddress(String);

impl PoolTokenAccountAddress {
    /// Creates a currency pool token account address.
    pub fn new(value: String) -> Result<Self, PoolTokenAccountAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(PoolTokenAccountAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(PoolTokenAccountAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the pool account address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PoolTokenAccountAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for PoolTokenAccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for PoolTokenAccountAddress {
    type Err = PoolTokenAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for PoolTokenAccountAddress {
    type Error = PoolTokenAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for PoolTokenAccountAddress {
    type Error = PoolTokenAccountAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PoolTokenAccountAddress> for String {
    fn from(value: PoolTokenAccountAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PoolTokenAccountAddress, PoolTokenAccountAddressError};

    #[test]
    fn accepts_valid_pool_token_account_address() {
        let address = PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
            .expect("address should be valid");

        assert_eq!(address.value(), "Pool111111111111111111111111111111111111");
    }

    #[test]
    fn rejects_empty_pool_token_account_address() {
        let error = PoolTokenAccountAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, PoolTokenAccountAddressError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_pool_token_account_address() {
        let error =
            PoolTokenAccountAddress::try_from("Pool 111").expect_err("invalid address should fail");

        assert!(matches!(error, PoolTokenAccountAddressError::InvalidFormat));
    }
}
