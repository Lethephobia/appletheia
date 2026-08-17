use std::error::Error;

use thiserror::Error;

/// Error returned when a PostgreSQL row cannot materialize a public user fragment.
#[derive(Debug, Error)]
pub enum PgUserFragmentRowError {
    #[error("invalid user ID")]
    UserId(#[source] Box<dyn Error + Send + Sync>),
    #[error("invalid username")]
    Username(#[source] Box<dyn Error + Send + Sync>),
    #[error("invalid user display name")]
    DisplayName(#[source] Box<dyn Error + Send + Sync>),
    #[error("invalid user bio")]
    Bio(#[source] Box<dyn Error + Send + Sync>),
    #[error("invalid user picture")]
    Picture(#[source] Box<dyn Error + Send + Sync>),
    #[error("unknown public user status: {0}")]
    UnknownStatus(String),
    #[error("invalid source event ID")]
    SourceEventId(#[source] Box<dyn Error + Send + Sync>),
    #[error("invalid updated event ID")]
    UpdatedEventId(#[source] Box<dyn Error + Send + Sync>),
}
