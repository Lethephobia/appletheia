use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::AuthTokenExchangeCodeHashError;

/// Represents a hashed auth token exchange code persisted by the backend.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuthTokenExchangeCodeHash(String);

impl AuthTokenExchangeCodeHash {
    /// The encoded SHA-256 hash length with URL-safe base64 without padding.
    pub const LENGTH: usize = 43;

    /// Creates a new exchange code hash after validating its format.
    pub fn new(value: String) -> Result<Self, AuthTokenExchangeCodeHashError> {
        if value.len() != Self::LENGTH {
            return Err(AuthTokenExchangeCodeHashError::InvalidFormat);
        }

        if !value
            .chars()
            .all(|character| matches!(character, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
        {
            return Err(AuthTokenExchangeCodeHashError::InvalidFormat);
        }

        Ok(Self(value))
    }

    /// Returns the underlying hash string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for AuthTokenExchangeCodeHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for AuthTokenExchangeCodeHash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<AuthTokenExchangeCodeHash> for String {
    fn from(value: AuthTokenExchangeCodeHash) -> Self {
        value.0
    }
}
