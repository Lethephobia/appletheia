use thiserror::Error;

/// Errors that can occur when parsing a `PkceCodeVerifier`.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum PkceCodeVerifierError {
    #[error("code_verifier is too short: length={length} (min={min})")]
    TooShort { length: usize, min: usize },

    #[error("code_verifier is too long: length={length} (max={max})")]
    TooLong { length: usize, max: usize },

    #[error("code_verifier contains an invalid character at position {position}: {character:?}")]
    InvalidCharacter { character: char, position: usize },
}
