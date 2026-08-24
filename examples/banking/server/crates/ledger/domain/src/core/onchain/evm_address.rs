use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::EvmAddressError;

/// Represents the common canonical form of an EVM address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct EvmAddress([u8; 20]);

impl EvmAddress {
    /// Parses an EVM address and normalizes it to lower-case hexadecimal.
    pub fn new(value: String) -> Result<Self, EvmAddressError> {
        if value.is_empty() {
            return Err(EvmAddressError::Empty);
        }

        let hexadecimal = value.strip_prefix("0x").unwrap_or(&value);
        if hexadecimal.len() != 40 {
            return Err(EvmAddressError::InvalidFormat);
        }

        let mut bytes = [0; 20];
        for (index, pair) in hexadecimal.as_bytes().chunks_exact(2).enumerate() {
            let high = Self::decode_nibble(pair[0]).ok_or(EvmAddressError::InvalidFormat)?;
            let low = Self::decode_nibble(pair[1]).ok_or(EvmAddressError::InvalidFormat)?;
            bytes[index] = (high << 4) | low;
        }

        Ok(Self(bytes))
    }

    /// Creates an address from its decoded bytes.
    pub const fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    /// Returns the decoded address bytes.
    pub const fn as_bytes(&self) -> &[u8; 20] {
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

impl Serialize for EvmAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for EvmAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl Display for EvmAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("0x")?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for EvmAddress {
    type Err = EvmAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for EvmAddress {
    type Error = EvmAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for EvmAddress {
    type Error = EvmAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EvmAddress> for String {
    fn from(address: EvmAddress) -> Self {
        address.to_string()
    }
}

impl From<[u8; 20]> for EvmAddress {
    fn from(bytes: [u8; 20]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<EvmAddress> for [u8; 20] {
    fn from(address: EvmAddress) -> Self {
        address.0
    }
}

#[cfg(test)]
mod tests {
    use super::EvmAddress;

    #[test]
    fn stores_decoded_bytes_and_serializes_as_lowercase_hexadecimal() {
        let input = "0x11111111111111111111111111111111111111AA";
        let encoded = "0x11111111111111111111111111111111111111aa";
        let address = EvmAddress::try_from(input).expect("address should be valid");

        assert_eq!(address.as_bytes()[19], 0xaa);
        assert_eq!(address.to_string(), encoded);
        assert_eq!(
            serde_json::to_string(&address).expect("address should serialize"),
            format!("\"{encoded}\"")
        );
        assert_eq!(
            serde_json::from_str::<EvmAddress>(&format!("\"{encoded}\""))
                .expect("address should deserialize"),
            address
        );
    }
}
