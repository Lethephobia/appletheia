use thiserror::Error;

/// Describes why a search term could not be constructed.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum SearchTermError {
    #[error("search term must not be empty")]
    Empty,
}
