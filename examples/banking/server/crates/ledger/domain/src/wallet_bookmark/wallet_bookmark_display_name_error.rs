use thiserror::Error;

/// Describes why a wallet bookmark display name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum WalletBookmarkDisplayNameError {
    #[error("wallet bookmark display name cannot be empty")]
    Empty,

    #[error("wallet bookmark display name is too long")]
    TooLong,
}
