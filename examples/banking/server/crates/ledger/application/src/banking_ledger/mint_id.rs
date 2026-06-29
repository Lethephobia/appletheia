use std::fmt::{self, Display};
use std::str::FromStr;

use appletheia::domain::AggregateId;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use super::MintIdError;

/// Represents an on-chain mint ID.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MintId {
    value: String,
    bytes: [u8; 16],
}

impl MintId {
    const MAX_BYTES: usize = 32;

    /// Creates a mint ID.
    pub fn new(value: String) -> Result<Self, MintIdError> {
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(MintIdError::Empty);
        }

        if value.len() > Self::MAX_BYTES {
            return Err(MintIdError::TooLong);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(MintIdError::InvalidFormat);
        }

        let uuid = Uuid::parse_str(&value).map_err(|_| MintIdError::InvalidFormat)?;

        Ok(Self {
            value,
            bytes: *uuid.as_bytes(),
        })
    }

    /// Returns the mint ID value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the on-chain mint ID bytes.
    pub fn bytes(&self) -> [u8; 16] {
        self.bytes
    }
}

impl AsRef<str> for MintId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for MintId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for MintId {
    type Err = MintIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<CurrencyId> for MintId {
    type Error = MintIdError;

    fn try_from(value: CurrencyId) -> Result<Self, Self::Error> {
        Self::try_from(value.value().as_simple().to_string())
    }
}

impl TryFrom<&str> for MintId {
    type Error = MintIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for MintId {
    type Error = MintIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MintId> for String {
    fn from(value: MintId) -> Self {
        value.value
    }
}

impl Serialize for MintId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> Deserialize<'de> for MintId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use banking_ledger_domain::currency::CurrencyId;

    use super::{MintId, MintIdError};

    #[test]
    fn accepts_valid_mint_id() {
        let mint_id =
            MintId::try_from("00000000000000000000000000000000").expect("mint ID should be valid");

        assert_eq!(mint_id.value(), "00000000000000000000000000000000");
        assert_eq!(mint_id.bytes(), [0; 16]);
    }

    #[test]
    fn builds_from_currency_id() {
        let currency_id = CurrencyId::new();

        let mint_id = MintId::try_from(currency_id).expect("mint ID should be valid");

        assert_eq!(mint_id.value(), currency_id.value().as_simple().to_string());
    }

    #[test]
    fn rejects_empty_mint_id() {
        let error = MintId::try_from(" ").expect_err("empty mint ID should fail");

        assert!(matches!(error, MintIdError::Empty));
    }

    #[test]
    fn rejects_too_long_mint_id() {
        let error = MintId::try_from("000000000000000000000000000000000")
            .expect_err("too long mint ID should fail");

        assert!(matches!(error, MintIdError::TooLong));
    }

    #[test]
    fn rejects_whitespace_in_mint_id() {
        let error = MintId::try_from("seed value").expect_err("invalid mint ID should fail");

        assert!(matches!(error, MintIdError::InvalidFormat));
    }

    #[test]
    fn rejects_non_uuid_mint_id() {
        let error = MintId::try_from("not-a-uuid").expect_err("invalid mint ID should fail");

        assert!(matches!(error, MintIdError::InvalidFormat));
    }
}
