use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DepositNoteError {
    #[error("deposit note cannot be empty")]
    Empty,
    #[error("deposit note is too long")]
    TooLong,
}
