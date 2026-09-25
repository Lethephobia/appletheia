use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvmCallData(Vec<u8>);

impl EvmCallData {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

        let mut encoded = String::with_capacity(2 + self.0.len() * 2);
        encoded.push_str("0x");
        for byte in &self.0 {
            encoded.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            encoded.push(HEX_DIGITS[(byte & 0x0f) as usize] as char);
        }
        encoded
    }

    fn decode_hex(value: &str) -> Option<Vec<u8>> {
        let hexadecimal = value.strip_prefix("0x")?;
        if !hexadecimal.len().is_multiple_of(2) {
            return None;
        }

        hexadecimal
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let high = Self::decode_nibble(pair[0])?;
                let low = Self::decode_nibble(pair[1])?;
                Some((high << 4) | low)
            })
            .collect()
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

impl Serialize for EvmCallData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for EvmCallData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        Self::decode_hex(&encoded)
            .map(Self::from_bytes)
            .ok_or_else(|| serde::de::Error::custom("invalid EVM call data"))
    }
}

#[cfg(test)]
mod tests {
    use super::EvmCallData;

    #[test]
    fn stores_bytes_and_serializes_as_prefixed_hexadecimal() {
        let call_data = EvmCallData::from_bytes(vec![0x12, 0xab, 0xcd]);

        assert_eq!(call_data.as_bytes(), &[0x12, 0xab, 0xcd]);
        assert_eq!(call_data.to_hex(), "0x12abcd");
        assert_eq!(
            serde_json::to_string(&call_data).expect("call data should serialize"),
            "\"0x12abcd\""
        );
        assert_eq!(
            serde_json::from_str::<EvmCallData>("\"0x12ABCD\"")
                .expect("call data should deserialize"),
            call_data
        );
    }

    #[test]
    fn rejects_invalid_hexadecimal() {
        assert!(serde_json::from_str::<EvmCallData>("\"12abcd\"").is_err());
        assert!(serde_json::from_str::<EvmCallData>("\"0x123\"").is_err());
        assert!(serde_json::from_str::<EvmCallData>("\"0x12xz\"").is_err());
    }
}
