use thiserror::Error;

/// Describes why a wallet bookmark description is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum WalletBookmarkDescriptionError {
    #[error("wallet bookmark description cannot be empty")]
    Empty,

    #[error("wallet bookmark description is too long")]
    TooLong,
}
