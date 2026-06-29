use std::fmt::{self, Display};
use std::str::FromStr;

use banking_ledger_domain::payout_destination::PayoutDestinationTokenAccountOwnerAddress;
use serde::{Deserialize, Serialize};

use super::TokenAccountOwnerAddressError;

/// Represents a token account owner address passed to an on-chain gateway.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TokenAccountOwnerAddress(String);

impl TokenAccountOwnerAddress {
    /// Creates a token account owner address.
    pub fn new(value: String) -> Result<Self, TokenAccountOwnerAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(TokenAccountOwnerAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(TokenAccountOwnerAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the token account owner address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TokenAccountOwnerAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for TokenAccountOwnerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for TokenAccountOwnerAddress {
    type Err = TokenAccountOwnerAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for TokenAccountOwnerAddress {
    type Error = TokenAccountOwnerAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TokenAccountOwnerAddress {
    type Error = TokenAccountOwnerAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<TokenAccountOwnerAddress> for String {
    fn from(value: TokenAccountOwnerAddress) -> Self {
        value.0
    }
}

impl From<PayoutDestinationTokenAccountOwnerAddress> for TokenAccountOwnerAddress {
    fn from(value: PayoutDestinationTokenAccountOwnerAddress) -> Self {
        Self(value.into())
    }
}
