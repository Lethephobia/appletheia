use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvmQuantity([u8; 32]);

impl EvmQuantity {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn from_u64(value: u64) -> Self {
        let mut bytes = [0; 32];
        let value_bytes = value.to_be_bytes();
        let mut index = 0;
        while index < value_bytes.len() {
            bytes[24 + index] = value_bytes[index];
            index += 1;
        }
        Self(bytes)
    }

    pub const fn from_u128(value: u128) -> Self {
        let mut bytes = [0; 32];
        let value_bytes = value.to_be_bytes();
        let mut index = 0;
        while index < value_bytes.len() {
            bytes[16 + index] = value_bytes[index];
            index += 1;
        }
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

        let first_nonzero_position = self.0.iter().position(|byte| *byte != 0);
        let Some(first_nonzero_index) = first_nonzero_position else {
            return "0x0".to_owned();
        };
        let first = self.0[first_nonzero_index];
        let mut encoded = String::with_capacity(2 + (32 - first_nonzero_index) * 2);
        encoded.push_str("0x");
        if first < 16 {
            encoded.push(HEX_DIGITS[first as usize] as char);
        } else {
            encoded.push(HEX_DIGITS[(first >> 4) as usize] as char);
            encoded.push(HEX_DIGITS[(first & 0x0f) as usize] as char);
        }
        for byte in &self.0[first_nonzero_index + 1..] {
            encoded.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            encoded.push(HEX_DIGITS[(byte & 0x0f) as usize] as char);
        }
        encoded
    }

    fn decode_hex(value: &str) -> Option<Self> {
        let hexadecimal = value.strip_prefix("0x")?;
        if hexadecimal.is_empty()
            || hexadecimal.len() > 64
            || (hexadecimal.len() > 1 && hexadecimal.starts_with('0'))
        {
            return None;
        }
        let mut bytes = [0; 32];
        let mut destination = 32 - hexadecimal.len().div_ceil(2);
        let mut source = 0;
        if !hexadecimal.len().is_multiple_of(2) {
            bytes[destination] = Self::decode_nibble(hexadecimal.as_bytes()[0])?;
            destination += 1;
            source = 1;
        }
        for pair in hexadecimal.as_bytes()[source..].chunks_exact(2) {
            bytes[destination] =
                (Self::decode_nibble(pair[0])? << 4) | Self::decode_nibble(pair[1])?;
            destination += 1;
        }
        Some(Self(bytes))
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

impl Serialize for EvmQuantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for EvmQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        Self::decode_hex(&encoded).ok_or_else(|| serde::de::Error::custom("invalid EVM quantity"))
    }
}

#[cfg(test)]
mod tests {
    use super::EvmQuantity;

    #[test]
    fn serializes_as_json_rpc_quantity() {
        assert_eq!(EvmQuantity::default().to_hex(), "0x0");
        assert_eq!(EvmQuantity::from_u64(0x012a).to_hex(), "0x12a");
        assert_eq!(
            serde_json::from_str::<EvmQuantity>("\"0x12A\"").expect("quantity should deserialize"),
            EvmQuantity::from_u64(0x012a)
        );
    }

    #[test]
    fn rejects_noncanonical_quantities() {
        assert!(serde_json::from_str::<EvmQuantity>("\"0x\"").is_err());
        assert!(serde_json::from_str::<EvmQuantity>("\"0x00\"").is_err());
        assert!(serde_json::from_str::<EvmQuantity>("\"12\"").is_err());
    }
}
