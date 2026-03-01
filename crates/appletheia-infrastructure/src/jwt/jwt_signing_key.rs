use std::fmt;

use jsonwebtoken::{Algorithm, EncodingKey};

use crate::jwt::{JwkKeyId, JwtSigningKeyError};

#[derive(Clone, Eq, PartialEq)]
pub enum JwtSigningKey {
    Rs256 {
        key_id: JwkKeyId,
        private_key_pem: Vec<u8>,
    },
    EdDsa {
        key_id: JwkKeyId,
        private_key_pem: Vec<u8>,
    },
}

impl JwtSigningKey {
    pub fn key_id(&self) -> &JwkKeyId {
        match self {
            Self::Rs256 { key_id, .. } => key_id,
            Self::EdDsa { key_id, .. } => key_id,
        }
    }

    pub fn algorithm(&self) -> Algorithm {
        match self {
            Self::Rs256 { .. } => Algorithm::RS256,
            Self::EdDsa { .. } => Algorithm::EdDSA,
        }
    }

    pub fn try_into_encoding_key(&self) -> Result<EncodingKey, JwtSigningKeyError> {
        match self {
            Self::Rs256 {
                private_key_pem, ..
            } => EncodingKey::from_rsa_pem(private_key_pem),
            Self::EdDsa {
                private_key_pem, ..
            } => EncodingKey::from_ed_pem(private_key_pem),
        }
        .map_err(JwtSigningKeyError::Backend)
    }
}

impl fmt::Debug for JwtSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rs256 { key_id, .. } => f
                .debug_struct("JwtSigningKey::Rs256")
                .field("key_id", key_id)
                .field("private_key_pem", &"[REDACTED]")
                .finish(),
            Self::EdDsa { key_id, .. } => f
                .debug_struct("JwtSigningKey::EdDsa")
                .field("key_id", key_id)
                .field("private_key_pem", &"[REDACTED]")
                .finish(),
        }
    }
}
