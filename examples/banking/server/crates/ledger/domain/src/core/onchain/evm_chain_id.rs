use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Identifies an EVM-compatible chain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EvmChainId(u64);

impl EvmChainId {
    /// Creates an EVM chain ID.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric chain ID.
    pub const fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for EvmChainId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl Display for EvmChainId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::EvmChainId;

    #[test]
    fn retains_the_numeric_chain_id() {
        let chain_id = EvmChainId::new(11_155_111);

        assert_eq!(chain_id.value(), 11_155_111);
        assert_eq!(chain_id.to_string(), "11155111");
    }
}
