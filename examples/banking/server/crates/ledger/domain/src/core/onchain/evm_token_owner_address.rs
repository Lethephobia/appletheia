use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{EvmAddress, EvmAddressError};

/// Identifies an EVM address that owns external tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EvmTokenOwnerAddress(EvmAddress);

impl EvmTokenOwnerAddress {
    /// Wraps a validated EVM address with its Ledger role.
    pub fn new(address: EvmAddress) -> Self {
        Self(address)
    }

    /// Returns the common EVM address.
    pub fn address(&self) -> &EvmAddress {
        &self.0
    }
}

impl Display for EvmTokenOwnerAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for EvmTokenOwnerAddress {
    type Err = EvmAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(EvmAddress::from_str(value)?))
    }
}

impl From<EvmAddress> for EvmTokenOwnerAddress {
    fn from(address: EvmAddress) -> Self {
        Self::new(address)
    }
}
