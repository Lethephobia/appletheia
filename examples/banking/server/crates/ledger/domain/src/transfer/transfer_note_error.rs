use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TransferNoteError {
    #[error("transfer note cannot be empty")]
    Empty,
    #[error("transfer note is too long")]
    TooLong,
}
