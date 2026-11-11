use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::SolanaTransactionSignatureError;

/// Identifies a Solana transaction by its canonical signature.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SolanaTransactionSignature([u8; 64]);

impl SolanaTransactionSignature {
    /// Parses a Solana transaction signature.
    pub fn new(value: String) -> Result<Self, SolanaTransactionSignatureError> {
        if value.is_empty() {
            return Err(SolanaTransactionSignatureError::Empty);
        }

        let decoded = bs58::decode(&value)
            .into_vec()
            .map_err(|_| SolanaTransactionSignatureError::InvalidEncoding)?;
        let bytes = decoded
            .try_into()
            .map_err(|_| SolanaTransactionSignatureError::InvalidByteLength)?;

        Ok(Self(bytes))
    }

    /// Creates a transaction signature from its decoded bytes.
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Returns the decoded transaction-signature bytes.
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Serialize for SolanaTransactionSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SolanaTransactionSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl Display for SolanaTransactionSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&bs58::encode(self.0).into_string())
    }
}

impl FromStr for SolanaTransactionSignature {
    type Err = SolanaTransactionSignatureError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for SolanaTransactionSignature {
    type Error = SolanaTransactionSignatureError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SolanaTransactionSignature {
    type Error = SolanaTransactionSignatureError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<[u8; 64]> for SolanaTransactionSignature {
    fn from(bytes: [u8; 64]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<SolanaTransactionSignature> for [u8; 64] {
    fn from(signature: SolanaTransactionSignature) -> Self {
        signature.0
    }
}

#[cfg(test)]
mod tests {
    use super::SolanaTransactionSignature;

    #[test]
    fn stores_decoded_bytes_and_serializes_as_base58() {
        let encoded = bs58::encode([1_u8; 64]).into_string();
        let signature = SolanaTransactionSignature::try_from(encoded.as_str())
            .expect("signature should be valid");

        assert_eq!(signature.as_bytes(), &[1; 64]);
        assert_eq!(signature.to_string(), encoded);
        assert_eq!(
            serde_json::to_string(&signature).expect("signature should serialize"),
            format!("\"{encoded}\"")
        );
        assert_eq!(
            serde_json::from_str::<SolanaTransactionSignature>(&format!("\"{encoded}\""))
                .expect("signature should deserialize"),
            signature
        );
    }
}
