use thiserror::Error;

/// Error returned while mapping PostgreSQL image columns into currency image references.
#[derive(Debug, Error)]
pub(crate) enum PgCurrencyImageRefColumnsError {
    #[error("unknown currency image type: {0}")]
    UnknownType(String),

    #[error("inconsistent currency image columns")]
    InconsistentColumns,

    #[error("invalid currency image object name")]
    ObjectName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid currency image external URL")]
    ExternalUrl(#[source] Box<dyn std::error::Error + Send + Sync>),
}
