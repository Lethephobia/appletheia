use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{SolanaAccountAddress, SolanaAccountAddressError};

/// Identifies the owner of a Solana token account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SolanaTokenAccountOwnerAddress(SolanaAccountAddress);

impl SolanaTokenAccountOwnerAddress {
    /// Wraps a validated Solana account address with its Ledger role.
    pub fn new(address: SolanaAccountAddress) -> Self {
        Self(address)
    }

    /// Returns the common Solana account address.
    pub fn address(&self) -> &SolanaAccountAddress {
        &self.0
    }
}

impl Display for SolanaTokenAccountOwnerAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for SolanaTokenAccountOwnerAddress {
    type Err = SolanaAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(SolanaAccountAddress::from_str(value)?))
    }
}

impl TryFrom<&str> for SolanaTokenAccountOwnerAddress {
    type Error = SolanaAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<SolanaAccountAddress> for SolanaTokenAccountOwnerAddress {
    fn from(address: SolanaAccountAddress) -> Self {
        Self::new(address)
    }
}
