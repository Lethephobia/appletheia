use super::{AuthTokenExchangeCode, AuthTokenExchangeCodeHash, AuthTokenExchangeCodeHasherError};

/// Hashes auth token exchange codes for persistence and lookup.
pub trait AuthTokenExchangeCodeHasher: Send + Sync {
    /// Hashes the provided exchange code.
    fn hash_code(
        &self,
        code: &AuthTokenExchangeCode,
    ) -> Result<AuthTokenExchangeCodeHash, AuthTokenExchangeCodeHasherError>;
}
