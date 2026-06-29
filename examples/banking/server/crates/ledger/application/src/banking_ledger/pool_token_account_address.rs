use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::currency::{
    CurrencyPoolTokenAccountAddress, CurrencyPoolTokenAccountAddressError,
};
use serde::{Deserialize, Serialize};

use super::PoolTokenAccountAddressError;

/// Represents a pool token account address passed to an on-chain gateway.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PoolTokenAccountAddress(String);

impl PoolTokenAccountAddress {
    /// Creates a pool token account address.
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

    /// Returns the account address value.
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

impl TryFrom<PoolTokenAccountAddress> for CurrencyPoolTokenAccountAddress {
    type Error = CurrencyPoolTokenAccountAddressError;

    fn try_from(value: PoolTokenAccountAddress) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

impl From<PoolTokenAccountAddress> for String {
    fn from(value: PoolTokenAccountAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyPoolTokenAccountAddress;

    use super::{PoolTokenAccountAddress, PoolTokenAccountAddressError};

    #[test]
    fn accepts_valid_pool_token_account_address() {
        let address = PoolTokenAccountAddress::try_from("Account111111111111111111111111111111111")
            .expect("address should be valid");

        assert_eq!(address.value(), "Account111111111111111111111111111111111");
    }

    #[test]
    fn rejects_empty_pool_token_account_address() {
        let error = PoolTokenAccountAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, PoolTokenAccountAddressError::Empty));
    }

    #[test]
    fn rejects_whitespace_in_pool_token_account_address() {
        let error = PoolTokenAccountAddress::try_from("Account 111")
            .expect_err("invalid address should fail");

        assert!(matches!(error, PoolTokenAccountAddressError::InvalidFormat));
    }

    #[test]
    fn converts_to_domain_currency_pool_token_account_address() {
        let address = PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
            .expect("address should be valid");

        let domain_address = CurrencyPoolTokenAccountAddress::try_from(address)
            .expect("domain address should be valid");

        assert_eq!(
            domain_address.value(),
            "Pool111111111111111111111111111111111111"
        );
    }
}
