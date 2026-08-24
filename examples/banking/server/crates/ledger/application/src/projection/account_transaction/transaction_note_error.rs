use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TransactionNoteError {
    #[error("transaction note cannot be empty")]
    Empty,
    #[error("transaction note is too long")]
    TooLong,
}
