use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{SolanaAccountAddress, SolanaAccountAddressError};

/// Identifies a Solana account used as a token Mint Account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SolanaMintAccountAddress(SolanaAccountAddress);

impl SolanaMintAccountAddress {
    /// Wraps a validated Solana account address with its Ledger role.
    pub fn new(address: SolanaAccountAddress) -> Self {
        Self(address)
    }

    /// Returns the common Solana account address.
    pub fn address(&self) -> &SolanaAccountAddress {
        &self.0
    }
}

impl Display for SolanaMintAccountAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for SolanaMintAccountAddress {
    type Err = SolanaAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(SolanaAccountAddress::from_str(value)?))
    }
}

impl TryFrom<&str> for SolanaMintAccountAddress {
    type Error = SolanaAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<SolanaAccountAddress> for SolanaMintAccountAddress {
    fn from(address: SolanaAccountAddress) -> Self {
        Self::new(address)
    }
}
