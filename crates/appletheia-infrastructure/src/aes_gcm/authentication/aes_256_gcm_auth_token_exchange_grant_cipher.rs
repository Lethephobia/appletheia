use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use appletheia_application::authentication::{
    AuthTokenExchangeGrant, AuthTokenExchangeGrantCipher, AuthTokenExchangeGrantCipherError,
    EncryptedAuthTokenExchangeGrant,
};

use super::aes_256_gcm_auth_token_exchange_grant_cipher_error::Aes256GcmAuthTokenExchangeGrantCipherError;
use super::auth_token_exchange_grant_json::AuthTokenExchangeGrantJson;

/// Encrypts exchange grants with AES-256-GCM.
#[derive(Clone)]
pub struct Aes256GcmAuthTokenExchangeGrantCipher {
    cipher: Aes256Gcm,
}

impl std::fmt::Debug for Aes256GcmAuthTokenExchangeGrantCipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Aes256GcmAuthTokenExchangeGrantCipher([REDACTED])")
    }
}

impl Aes256GcmAuthTokenExchangeGrantCipher {
    const NONCE_LENGTH: usize = 12;

    pub fn new(cipher: Aes256Gcm) -> Self {
        Self { cipher }
    }

    pub fn from_key_bytes(
        key_bytes: [u8; 32],
    ) -> Result<Self, Aes256GcmAuthTokenExchangeGrantCipherError> {
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(Aes256GcmAuthTokenExchangeGrantCipherError::InvalidKeyBytes)?;
        Ok(Self::new(cipher))
    }
}

impl AuthTokenExchangeGrantCipher for Aes256GcmAuthTokenExchangeGrantCipher {
    async fn encrypt(
        &self,
        grant: &AuthTokenExchangeGrant,
    ) -> Result<EncryptedAuthTokenExchangeGrant, AuthTokenExchangeGrantCipherError> {
        let plaintext = serde_json::to_vec(&AuthTokenExchangeGrantJson::from_grant(grant))
            .map_err(|source| AuthTokenExchangeGrantCipherError::Backend(Box::new(source)))?;
        let mut nonce_bytes = [0u8; Self::NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::from(nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|source| AuthTokenExchangeGrantCipherError::Backend(Box::new(source)))?;

        let mut encrypted = Vec::with_capacity(Self::NONCE_LENGTH + ciphertext.len());
        encrypted.extend_from_slice(&nonce_bytes);
        encrypted.extend_from_slice(&ciphertext);
        Ok(EncryptedAuthTokenExchangeGrant::new(encrypted))
    }

    async fn decrypt(
        &self,
        encrypted_grant: &EncryptedAuthTokenExchangeGrant,
    ) -> Result<AuthTokenExchangeGrant, AuthTokenExchangeGrantCipherError> {
        let ciphertext = encrypted_grant.as_bytes();
        if ciphertext.len() < Self::NONCE_LENGTH {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ciphertext is shorter than nonce length",
            );
            return Err(AuthTokenExchangeGrantCipherError::Backend(Box::new(source)));
        }

        let (nonce_bytes, encrypted_payload) = ciphertext.split_at(Self::NONCE_LENGTH);
        let nonce_bytes: [u8; Self::NONCE_LENGTH] = nonce_bytes.try_into().map_err(|_| {
            let source =
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid nonce length");
            AuthTokenExchangeGrantCipherError::Backend(Box::new(source))
        })?;
        let nonce = Nonce::from(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(&nonce, encrypted_payload)
            .map_err(|source| AuthTokenExchangeGrantCipherError::Backend(Box::new(source)))?;
        let json: AuthTokenExchangeGrantJson = serde_json::from_slice(&plaintext)
            .map_err(|source| AuthTokenExchangeGrantCipherError::Backend(Box::new(source)))?;

        json.into_grant()
            .map_err(AuthTokenExchangeGrantCipherError::Backend)
    }
}
