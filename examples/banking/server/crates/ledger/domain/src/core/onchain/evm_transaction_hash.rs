use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::EvmTransactionHashError;

/// Identifies an EVM transaction by its canonical hash.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct EvmTransactionHash([u8; 32]);

impl EvmTransactionHash {
    /// Parses an EVM transaction hash and normalizes it to lower-case hexadecimal.
    pub fn new(value: String) -> Result<Self, EvmTransactionHashError> {
        if value.is_empty() {
            return Err(EvmTransactionHashError::Empty);
        }

        let hexadecimal = value.strip_prefix("0x").unwrap_or(&value);
        if hexadecimal.len() != 64 {
            return Err(EvmTransactionHashError::InvalidFormat);
        }

        let mut bytes = [0; 32];
        for (index, pair) in hexadecimal.as_bytes().chunks_exact(2).enumerate() {
            let high =
                Self::decode_nibble(pair[0]).ok_or(EvmTransactionHashError::InvalidFormat)?;
            let low = Self::decode_nibble(pair[1]).ok_or(EvmTransactionHashError::InvalidFormat)?;
            bytes[index] = (high << 4) | low;
        }

        Ok(Self(bytes))
    }

    /// Creates a transaction hash from its decoded bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the decoded transaction-hash bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    const fn decode_nibble(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
}

impl Serialize for EvmTransactionHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for EvmTransactionHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl Display for EvmTransactionHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("0x")?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for EvmTransactionHash {
    type Err = EvmTransactionHashError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for EvmTransactionHash {
    type Error = EvmTransactionHashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for EvmTransactionHash {
    type Error = EvmTransactionHashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<[u8; 32]> for EvmTransactionHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<EvmTransactionHash> for [u8; 32] {
    fn from(hash: EvmTransactionHash) -> Self {
        hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::EvmTransactionHash;

    #[test]
    fn stores_decoded_bytes_and_serializes_as_lowercase_hexadecimal() {
        let input = format!("0x{}AA", "11".repeat(31));
        let encoded = format!("0x{}aa", "11".repeat(31));
        let hash = EvmTransactionHash::try_from(input).expect("hash should be valid");

        assert_eq!(hash.as_bytes()[31], 0xaa);
        assert_eq!(hash.to_string(), encoded);
        assert_eq!(
            serde_json::to_string(&hash).expect("hash should serialize"),
            format!("\"{encoded}\"")
        );
        assert_eq!(
            serde_json::from_str::<EvmTransactionHash>(&format!("\"{encoded}\""))
                .expect("hash should deserialize"),
            hash
        );
    }
}
