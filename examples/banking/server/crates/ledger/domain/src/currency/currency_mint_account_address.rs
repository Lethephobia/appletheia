use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::CurrencyMintAccountAddressError;

/// Represents a currency mint account address.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyMintAccountAddress(String);

impl CurrencyMintAccountAddress {
    /// Creates a currency mint account address.
    pub fn new(value: String) -> Result<Self, CurrencyMintAccountAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(CurrencyMintAccountAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(CurrencyMintAccountAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the mint account address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CurrencyMintAccountAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for CurrencyMintAccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for CurrencyMintAccountAddress {
    type Err = CurrencyMintAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for CurrencyMintAccountAddress {
    type Error = CurrencyMintAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for CurrencyMintAccountAddress {
    type Error = CurrencyMintAccountAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CurrencyMintAccountAddress> for String {
    fn from(value: CurrencyMintAccountAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrencyMintAccountAddress, CurrencyMintAccountAddressError};

    #[test]
    fn accepts_valid_mint_account_address() {
        let address =
            CurrencyMintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("address should be valid");

        assert_eq!(address.value(), "Mint111111111111111111111111111111111111");
    }

    #[test]
    fn rejects_empty_mint_account_address() {
        let error =
            CurrencyMintAccountAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, CurrencyMintAccountAddressError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_mint_account_address() {
        let error = CurrencyMintAccountAddress::try_from("Mint 111")
            .expect_err("invalid address should fail");

        assert!(matches!(
            error,
            CurrencyMintAccountAddressError::InvalidFormat
        ));
    }
}
