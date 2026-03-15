use std::fmt;
use std::str::FromStr;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::{PkceCodeChallenge, PkceCodeChallengeMethod, PkceCodeVerifierError};

/// Represents a PKCE code verifier.
#[derive(Clone, PartialEq, Eq)]
pub struct PkceCodeVerifier(String);

impl PkceCodeVerifier {
    /// Generates a new verifier.
    pub fn new() -> Self {
        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(Uuid::new_v4().as_bytes());
        bytes[16..].copy_from_slice(Uuid::new_v4().as_bytes());
        let encoded = URL_SAFE_NO_PAD.encode(bytes);
        Self(encoded)
    }

    /// Returns the raw verifier value.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Derives the challenge for the configured method.
    pub fn to_code_challenge(&self, method: PkceCodeChallengeMethod) -> PkceCodeChallenge {
        match method {
            PkceCodeChallengeMethod::Plain => PkceCodeChallenge::new(self.0.clone()),
            PkceCodeChallengeMethod::S256 => {
                let digest = Sha256::digest(self.0.as_bytes());
                let encoded = URL_SAFE_NO_PAD.encode(digest);
                PkceCodeChallenge::new(encoded)
            }
        }
    }

    fn validate(value: &str) -> Result<(), PkceCodeVerifierError> {
        const MIN_LEN: usize = 43;
        const MAX_LEN: usize = 128;

        let length = value.len();
        if length < MIN_LEN {
            return Err(PkceCodeVerifierError::TooShort {
                length,
                min: MIN_LEN,
            });
        }
        if length > MAX_LEN {
            return Err(PkceCodeVerifierError::TooLong {
                length,
                max: MAX_LEN,
            });
        }

        for (position, character) in value.chars().enumerate() {
            let is_valid =
                matches!(character, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' | '_' | '~');
            if !is_valid {
                return Err(PkceCodeVerifierError::InvalidCharacter {
                    character,
                    position,
                });
            }
        }

        Ok(())
    }
}

impl Default for PkceCodeVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for PkceCodeVerifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PkceCodeVerifier([REDACTED])")
    }
}

impl FromStr for PkceCodeVerifier {
    type Err = PkceCodeVerifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::validate(value)?;
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<String> for PkceCodeVerifier {
    type Error = PkceCodeVerifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{PkceCodeVerifier, PkceCodeVerifierError};

    #[test]
    fn new_generates_valid_code_verifier() {
        let verifier = PkceCodeVerifier::new();
        assert!(verifier.value().len() >= 43);
        assert!(verifier.value().len() <= 128);
        PkceCodeVerifier::from_str(verifier.value()).expect("generated verifier should be valid");
    }

    #[test]
    fn from_str_rejects_too_short() {
        let input = "a".repeat(42);
        let error =
            PkceCodeVerifier::from_str(&input).expect_err("should reject too-short verifier");
        assert!(matches!(error, PkceCodeVerifierError::TooShort { .. }));
    }

    #[test]
    fn from_str_rejects_invalid_character() {
        let mut input = "a".repeat(43);
        input.replace_range(10..11, "!");
        let error =
            PkceCodeVerifier::from_str(&input).expect_err("should reject invalid character");
        assert!(matches!(
            error,
            PkceCodeVerifierError::InvalidCharacter { .. }
        ));
    }
}
