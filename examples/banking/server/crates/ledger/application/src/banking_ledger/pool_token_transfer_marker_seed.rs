use std::fmt::{self, Display};
use std::str::FromStr;

use appletheia::domain::AggregateId;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

use super::PoolTokenTransferMarkerSeedError;

/// Represents a deterministic seed used to create a marker account for pool token transfers.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PoolTokenTransferMarkerSeed(String);

impl PoolTokenTransferMarkerSeed {
    const MAX_BYTES: usize = 32;

    /// Creates a pool token transfer marker seed.
    pub fn new(value: String) -> Result<Self, PoolTokenTransferMarkerSeedError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(PoolTokenTransferMarkerSeedError::Empty);
        }

        if value.len() > Self::MAX_BYTES {
            return Err(PoolTokenTransferMarkerSeedError::TooLong);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(PoolTokenTransferMarkerSeedError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the marker seed value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PoolTokenTransferMarkerSeed {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for PoolTokenTransferMarkerSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for PoolTokenTransferMarkerSeed {
    type Err = PoolTokenTransferMarkerSeedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<WithdrawalId> for PoolTokenTransferMarkerSeed {
    type Error = PoolTokenTransferMarkerSeedError;

    fn try_from(value: WithdrawalId) -> Result<Self, Self::Error> {
        Self::try_from(value.value().as_simple().to_string())
    }
}

impl TryFrom<&str> for PoolTokenTransferMarkerSeed {
    type Error = PoolTokenTransferMarkerSeedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for PoolTokenTransferMarkerSeed {
    type Error = PoolTokenTransferMarkerSeedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PoolTokenTransferMarkerSeed> for String {
    fn from(value: PoolTokenTransferMarkerSeed) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use banking_ledger_domain::withdrawal::WithdrawalId;

    use super::{PoolTokenTransferMarkerSeed, PoolTokenTransferMarkerSeedError};

    #[test]
    fn accepts_valid_pool_token_transfer_marker_seed() {
        let seed = PoolTokenTransferMarkerSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");

        assert_eq!(seed.value(), "00000000000000000000000000000000");
    }

    #[test]
    fn builds_from_withdrawal_id() {
        let withdrawal_id = WithdrawalId::new();

        let seed =
            PoolTokenTransferMarkerSeed::try_from(withdrawal_id).expect("seed should be valid");

        assert_eq!(seed.value(), withdrawal_id.value().as_simple().to_string());
    }

    #[test]
    fn rejects_empty_pool_token_transfer_marker_seed() {
        let error = PoolTokenTransferMarkerSeed::try_from(" ").expect_err("empty seed should fail");

        assert!(matches!(error, PoolTokenTransferMarkerSeedError::Empty));
    }

    #[test]
    fn rejects_too_long_pool_token_transfer_marker_seed() {
        let error = PoolTokenTransferMarkerSeed::try_from("000000000000000000000000000000000")
            .expect_err("too long seed should fail");

        assert!(matches!(error, PoolTokenTransferMarkerSeedError::TooLong));
    }

    #[test]
    fn rejects_whitespace_in_pool_token_transfer_marker_seed() {
        let error = PoolTokenTransferMarkerSeed::try_from("seed value")
            .expect_err("invalid seed should fail");

        assert!(matches!(
            error,
            PoolTokenTransferMarkerSeedError::InvalidFormat
        ));
    }
}
