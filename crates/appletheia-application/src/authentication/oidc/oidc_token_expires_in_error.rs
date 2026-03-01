use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcTokenExpiresInError {
    #[error("expires_in must be positive")]
    NonPositive,

    #[error("expires_in is too large")]
    TooLarge,
}
