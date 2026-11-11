use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::SolanaAccountAddressError;

/// Represents the common encoded form of a Solana account address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SolanaAccountAddress([u8; 32]);

impl SolanaAccountAddress {
    /// Parses and validates a canonical Solana account address.
    pub fn new(value: String) -> Result<Self, SolanaAccountAddressError> {
        if value.is_empty() {
            return Err(SolanaAccountAddressError::Empty);
        }

        let decoded = bs58::decode(&value)
            .into_vec()
            .map_err(|_| SolanaAccountAddressError::InvalidEncoding)?;
        let bytes = decoded
            .try_into()
            .map_err(|_| SolanaAccountAddressError::InvalidByteLength)?;

        Ok(Self(bytes))
    }

    /// Creates an address from its decoded bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the decoded account-address bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Serialize for SolanaAccountAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SolanaAccountAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl Display for SolanaAccountAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&bs58::encode(self.0).into_string())
    }
}

impl FromStr for SolanaAccountAddress {
    type Err = SolanaAccountAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for SolanaAccountAddress {
    type Error = SolanaAccountAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SolanaAccountAddress {
    type Error = SolanaAccountAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SolanaAccountAddress> for String {
    fn from(address: SolanaAccountAddress) -> Self {
        address.to_string()
    }
}

impl From<[u8; 32]> for SolanaAccountAddress {
    fn from(bytes: [u8; 32]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<SolanaAccountAddress> for [u8; 32] {
    fn from(address: SolanaAccountAddress) -> Self {
        address.0
    }
}

#[cfg(test)]
mod tests {
    use super::SolanaAccountAddress;

    #[test]
    fn stores_decoded_bytes_and_serializes_as_base58() {
        let encoded = "11111111111111111111111111111111";
        let address = SolanaAccountAddress::try_from(encoded).expect("address should be valid");

        assert_eq!(address.as_bytes(), &[0; 32]);
        assert_eq!(address.to_string(), encoded);
        assert_eq!(
            serde_json::to_string(&address).expect("address should serialize"),
            format!("\"{encoded}\"")
        );
        assert_eq!(
            serde_json::from_str::<SolanaAccountAddress>(&format!("\"{encoded}\""))
                .expect("address should deserialize"),
            address
        );
    }
}
