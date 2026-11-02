use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaPreparedDepositTransaction(Vec<u8>);

impl SolanaPreparedDepositTransaction {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.0)
    }
}

impl Serialize for SolanaPreparedDepositTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_base64())
    }
}

impl<'de> Deserialize<'de> for SolanaPreparedDepositTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        BASE64_STANDARD
            .decode(encoded)
            .map(Self::from_bytes)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::SolanaPreparedDepositTransaction;

    #[test]
    fn stores_bytes_and_converts_them_to_base64() {
        let transaction = SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec());

        assert_eq!(transaction.as_bytes(), b"solana");
        assert_eq!(transaction.to_base64(), "c29sYW5h");
    }
}
