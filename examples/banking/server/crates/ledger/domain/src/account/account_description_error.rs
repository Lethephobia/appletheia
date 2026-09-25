use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AccountDescriptionError {
    #[error("account description cannot be empty")]
    Empty,
    #[error("account description is too long")]
    TooLong,
}
