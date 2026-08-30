use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Erc3009ReceiveAuthorizationSignature([u8; 65]);

impl Erc3009ReceiveAuthorizationSignature {
    pub const fn from_bytes(bytes: [u8; 65]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 65] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

        let mut encoded = String::with_capacity(132);
        encoded.push_str("0x");
        for byte in self.0 {
            encoded.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            encoded.push(HEX_DIGITS[(byte & 0x0f) as usize] as char);
        }
        encoded
    }

    fn decode_hex(value: &str) -> Option<Self> {
        let hexadecimal = value.strip_prefix("0x")?;
        if hexadecimal.len() != 130 {
            return None;
        }

        let bytes = hexadecimal
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let high = Self::decode_nibble(pair[0])?;
                let low = Self::decode_nibble(pair[1])?;
                Some((high << 4) | low)
            })
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()?;
        Some(Self::from_bytes(bytes))
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

impl Serialize for Erc3009ReceiveAuthorizationSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Erc3009ReceiveAuthorizationSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        Self::decode_hex(&encoded).ok_or_else(|| {
            serde::de::Error::custom("invalid ERC-3009 receive authorization signature")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Erc3009ReceiveAuthorizationSignature;

    #[test]
    fn stores_bytes_and_serializes_as_prefixed_hexadecimal() {
        let signature = Erc3009ReceiveAuthorizationSignature::from_bytes([0xab; 65]);
        let encoded = format!("\"0x{}\"", "ab".repeat(65));

        assert_eq!(signature.as_bytes(), &[0xab; 65]);
        assert_eq!(signature.to_hex(), format!("0x{}", "ab".repeat(65)));
        assert_eq!(
            serde_json::from_str::<Erc3009ReceiveAuthorizationSignature>(&encoded)
                .expect("signature should deserialize"),
            signature
        );
    }

    #[test]
    fn rejects_invalid_signature_length() {
        assert!(serde_json::from_str::<Erc3009ReceiveAuthorizationSignature>("\"0x12\"").is_err());
    }
}
