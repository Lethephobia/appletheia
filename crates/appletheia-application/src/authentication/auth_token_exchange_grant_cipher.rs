use super::{
    AuthTokenExchangeGrant, AuthTokenExchangeGrantCipherError, EncryptedAuthTokenExchangeGrant,
};

/// Encrypts and decrypts sensitive exchange grants before persistence.
#[allow(async_fn_in_trait)]
pub trait AuthTokenExchangeGrantCipher: Send + Sync {
    /// Encrypts an exchange grant for persistence.
    async fn encrypt(
        &self,
        grant: &AuthTokenExchangeGrant,
    ) -> Result<EncryptedAuthTokenExchangeGrant, AuthTokenExchangeGrantCipherError>;

    /// Decrypts an exchange grant loaded from persistence.
    async fn decrypt(
        &self,
        encrypted_grant: &EncryptedAuthTokenExchangeGrant,
    ) -> Result<AuthTokenExchangeGrant, AuthTokenExchangeGrantCipherError>;
}
