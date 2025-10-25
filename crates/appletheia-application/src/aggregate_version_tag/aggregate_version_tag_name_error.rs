use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateVersionTagNameError {
    #[error("name is empty")]
    Empty,

    #[error("name is too long: {0}, max length: {1}")]
    TooLong(usize, usize),
}
