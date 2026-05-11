use thiserror::Error;

/// Errors that can occur when parsing a PKCE code challenge method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum PkceCodeChallengeMethodError {
    #[error("invalid pkce code challenge method")]
    Invalid,
}
