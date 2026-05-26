use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::PayoutDestinationTokenAccountOwnerAddressError;

/// Represents a payout destination token account owner address.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PayoutDestinationTokenAccountOwnerAddress(String);

impl PayoutDestinationTokenAccountOwnerAddress {
    /// Creates a payout destination token account owner address.
    pub fn new(value: String) -> Result<Self, PayoutDestinationTokenAccountOwnerAddressError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(PayoutDestinationTokenAccountOwnerAddressError::Empty);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(PayoutDestinationTokenAccountOwnerAddressError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the payout destination token account owner address value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PayoutDestinationTokenAccountOwnerAddress {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for PayoutDestinationTokenAccountOwnerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for PayoutDestinationTokenAccountOwnerAddress {
    type Err = PayoutDestinationTokenAccountOwnerAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for PayoutDestinationTokenAccountOwnerAddress {
    type Error = PayoutDestinationTokenAccountOwnerAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for PayoutDestinationTokenAccountOwnerAddress {
    type Error = PayoutDestinationTokenAccountOwnerAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PayoutDestinationTokenAccountOwnerAddress> for String {
    fn from(value: PayoutDestinationTokenAccountOwnerAddress) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PayoutDestinationTokenAccountOwnerAddress, PayoutDestinationTokenAccountOwnerAddressError,
    };

    #[test]
    fn accepts_valid_payout_destination_token_account_owner_address() {
        let address = PayoutDestinationTokenAccountOwnerAddress::try_from("wallet-123")
            .expect("address should be valid");

        assert_eq!(address.value(), "wallet-123");
    }

    #[test]
    fn rejects_empty_payout_destination_token_account_owner_address() {
        let error = PayoutDestinationTokenAccountOwnerAddress::try_from(" ")
            .expect_err("empty address should fail");

        assert!(matches!(
            error,
            PayoutDestinationTokenAccountOwnerAddressError::Empty
        ));
    }

    #[test]
    fn rejects_invalid_payout_destination_token_account_owner_address() {
        let error = PayoutDestinationTokenAccountOwnerAddress::try_from("wallet 123")
            .expect_err("invalid address should fail");

        assert!(matches!(
            error,
            PayoutDestinationTokenAccountOwnerAddressError::InvalidFormat
        ));
    }
}
