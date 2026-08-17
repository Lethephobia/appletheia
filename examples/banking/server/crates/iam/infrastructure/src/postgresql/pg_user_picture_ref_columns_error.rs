use thiserror::Error;

/// Error returned while mapping PostgreSQL picture columns into user picture references.
#[derive(Debug, Error)]
pub(crate) enum PgUserPictureRefColumnsError {
    #[error("unknown user picture type: {0}")]
    UnknownType(String),

    #[error("inconsistent user picture columns")]
    InconsistentColumns,

    #[error("invalid user picture object name")]
    ObjectName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user picture external URL")]
    ExternalUrl(#[source] Box<dyn std::error::Error + Send + Sync>),
}
