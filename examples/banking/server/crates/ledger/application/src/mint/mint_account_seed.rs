use std::fmt::{self, Display};
use std::str::FromStr;

use appletheia::domain::AggregateId;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

use super::MintAccountSeedError;

/// Represents a deterministic seed used by an on-chain gateway to derive a mint account.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MintAccountSeed(String);

impl MintAccountSeed {
    const MAX_BYTES: usize = 32;

    /// Creates a mint account seed.
    pub fn new(value: String) -> Result<Self, MintAccountSeedError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintAccountSeedError::Empty);
        }

        if value.len() > Self::MAX_BYTES {
            return Err(MintAccountSeedError::TooLong);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(MintAccountSeedError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the mint account seed value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MintAccountSeed {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintAccountSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for MintAccountSeed {
    type Err = MintAccountSeedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<CurrencyId> for MintAccountSeed {
    type Error = MintAccountSeedError;

    fn try_from(value: CurrencyId) -> Result<Self, Self::Error> {
        Self::try_from(value.value().as_simple().to_string())
    }
}

impl TryFrom<&str> for MintAccountSeed {
    type Error = MintAccountSeedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintAccountSeed {
    type Error = MintAccountSeedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintAccountSeed> for String {
    fn from(value: MintAccountSeed) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use banking_ledger_domain::currency::CurrencyId;

    use super::{MintAccountSeed, MintAccountSeedError};

    #[test]
    fn accepts_valid_mint_account_seed() {
        let seed = MintAccountSeed::try_from("00000000000000000000000000000000")
            .expect("seed should be valid");

        assert_eq!(seed.value(), "00000000000000000000000000000000");
    }

    #[test]
    fn builds_from_currency_id() {
        let currency_id = CurrencyId::new();

        let seed = MintAccountSeed::try_from(currency_id).expect("seed should be valid");

        assert_eq!(seed.value(), currency_id.value().as_simple().to_string());
    }

    #[test]
    fn rejects_empty_mint_account_seed() {
        let error = MintAccountSeed::try_from(" ").expect_err("empty seed should fail");

        assert!(matches!(error, MintAccountSeedError::Empty));
    }

    #[test]
    fn rejects_too_long_mint_account_seed() {
        let error = MintAccountSeed::try_from("000000000000000000000000000000000")
            .expect_err("too long seed should fail");

        assert!(matches!(error, MintAccountSeedError::TooLong));
    }

    #[test]
    fn rejects_whitespace_in_mint_account_seed() {
        let error = MintAccountSeed::try_from("seed value").expect_err("invalid seed should fail");

        assert!(matches!(error, MintAccountSeedError::InvalidFormat));
    }
}
