use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest, Sha256};

use appletheia_application::authentication::{
    AuthTokenExchangeCode, AuthTokenExchangeCodeHash, AuthTokenExchangeCodeHasher,
    AuthTokenExchangeCodeHasherError,
};

/// Hashes auth token exchange codes with SHA-256 and URL-safe base64.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sha256AuthTokenExchangeCodeHasher;

impl Sha256AuthTokenExchangeCodeHasher {
    pub fn new() -> Self {
        Self
    }
}

impl AuthTokenExchangeCodeHasher for Sha256AuthTokenExchangeCodeHasher {
    fn hash_code(
        &self,
        code: &AuthTokenExchangeCode,
    ) -> Result<AuthTokenExchangeCodeHash, AuthTokenExchangeCodeHasherError> {
        let digest = Sha256::digest(code.value().as_bytes());
        let hash = URL_SAFE_NO_PAD.encode(digest);
        Ok(AuthTokenExchangeCodeHash::new(hash)?)
    }
}
