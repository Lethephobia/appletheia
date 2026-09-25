use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{EvmAddress, EvmAddressError};

/// Identifies an EVM contract used as a bound external token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EvmTokenContractAddress(EvmAddress);

impl EvmTokenContractAddress {
    /// Wraps a validated EVM address with its Ledger role.
    pub fn new(address: EvmAddress) -> Self {
        Self(address)
    }

    /// Returns the common EVM address.
    pub fn address(&self) -> &EvmAddress {
        &self.0
    }
}

impl Display for EvmTokenContractAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for EvmTokenContractAddress {
    type Err = EvmAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(EvmAddress::from_str(value)?))
    }
}

impl From<EvmAddress> for EvmTokenContractAddress {
    fn from(address: EvmAddress) -> Self {
        Self::new(address)
    }
}
