use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum WithdrawalNoteError {
    #[error("withdrawal note cannot be empty")]
    Empty,
    #[error("withdrawal note is too long")]
    TooLong,
}
