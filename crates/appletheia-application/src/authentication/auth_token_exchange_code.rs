use std::fmt;
use std::str::FromStr;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use uuid::Uuid;

use super::AuthTokenExchangeCodeError;

/// Represents a one-time code used to exchange for an auth token.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AuthTokenExchangeCode(String);

impl AuthTokenExchangeCode {
    /// Generates a new random exchange code.
    pub fn new() -> Self {
        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(Uuid::new_v4().as_bytes());
        bytes[16..].copy_from_slice(Uuid::new_v4().as_bytes());
        let encoded = URL_SAFE_NO_PAD.encode(bytes);
        Self(encoded)
    }

    /// Returns the raw code value.
    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), AuthTokenExchangeCodeError> {
        const MIN_LEN: usize = 43;
        const MAX_LEN: usize = 128;

        let length = value.len();
        if length < MIN_LEN {
            return Err(AuthTokenExchangeCodeError::TooShort {
                length,
                min: MIN_LEN,
            });
        }
        if length > MAX_LEN {
            return Err(AuthTokenExchangeCodeError::TooLong {
                length,
                max: MAX_LEN,
            });
        }

        for (position, character) in value.chars().enumerate() {
            let is_valid =
                matches!(character, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' | '_' | '~');
            if !is_valid {
                return Err(AuthTokenExchangeCodeError::InvalidCharacter {
                    character,
                    position,
                });
            }
        }

        Ok(())
    }
}

impl Default for AuthTokenExchangeCode {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for AuthTokenExchangeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AuthTokenExchangeCode([REDACTED])")
    }
}

impl FromStr for AuthTokenExchangeCode {
    type Err = AuthTokenExchangeCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for AuthTokenExchangeCode {
    type Error = AuthTokenExchangeCodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}
