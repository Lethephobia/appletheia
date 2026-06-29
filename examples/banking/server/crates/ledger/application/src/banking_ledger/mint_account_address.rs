use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::{
    CurrencyMintAccountAddress, CurrencyMintAccountAddressError,
};
use serde::{Deserialize, Serialize};

use super::MintAccountAddressError;

/// Represents an on-chain mint account address returned by an on-chain gateway.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintAccountAddress(String);

impl MintAccountAddress {
    /// Creates a mint account address.
    pub fn new(value: String) -> Result<Self, MintAccountAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintAccountAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(MintAccountAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the mint account address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintAccountAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintAccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for MintAccountAddress {
    type Err = MintAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for MintAccountAddress {
    type Error = MintAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintAccountAddress {
    type Error = MintAccountAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<MintAccountAddress> for CurrencyMintAccountAddress {
    type Error = CurrencyMintAccountAddressError;

    fn try_from(value: MintAccountAddress) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<MintAccountAddress> for String {
    fn from(value: MintAccountAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyMintAccountAddress;

    use super::{MintAccountAddress, MintAccountAddressError};

    #[test]
    fn accepts_valid_mint_account_address() {
        let address = MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
            .expect("address should be valid");

        assert_eq!(address.value(), "Mint111111111111111111111111111111111111");
    }

    #[test]
    fn converts_to_domain_currency_mint_account_address() {
        let address = MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
            .expect("address should be valid");

        let domain_address =
            CurrencyMintAccountAddress::try_from(address).expect("domain address should be valid");

        assert_eq!(
            domain_address.value(),
            "Mint111111111111111111111111111111111111"
        );
    }

    #[test]
    fn rejects_empty_mint_account_address() {
        let error = MintAccountAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, MintAccountAddressError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_mint_account_address() {
        let error =
            MintAccountAddress::try_from("Mint 111").expect_err("invalid address should fail");

        assert!(matches!(error, MintAccountAddressError::InvalidFormat));
    }
}
