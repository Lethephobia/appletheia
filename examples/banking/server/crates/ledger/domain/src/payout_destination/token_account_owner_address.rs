use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::TokenAccountOwnerAddressError;

/// Represents a payout destination token account owner address.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TokenAccountOwnerAddress(String);

impl TokenAccountOwnerAddress {
    /// Creates a payout destination token account owner address.
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

    /// Returns the payout destination token account owner address value.
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

#[cfg(test)]
mod tests {
    use super::{TokenAccountOwnerAddress, TokenAccountOwnerAddressError};

    #[test]
    fn accepts_valid_token_account_owner_address() {
        let address =
            TokenAccountOwnerAddress::try_from("wallet-123").expect("address should be valid");

        assert_eq!(address.value(), "wallet-123");
    }

    #[test]
    fn rejects_empty_token_account_owner_address() {
        let error = TokenAccountOwnerAddress::try_from(" ").expect_err("empty address should fail");

        assert!(matches!(error, TokenAccountOwnerAddressError::Empty));
    }

    #[test]
    fn rejects_invalid_token_account_owner_address() {
        let error = TokenAccountOwnerAddress::try_from("wallet 123")
            .expect_err("invalid address should fail");

        assert!(matches!(
            error,
            TokenAccountOwnerAddressError::InvalidFormat
        ));
    }
}
