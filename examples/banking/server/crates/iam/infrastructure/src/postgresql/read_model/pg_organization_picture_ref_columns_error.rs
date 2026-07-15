use thiserror::Error;

/// Error returned while mapping PostgreSQL picture columns into organization picture references.
#[derive(Debug, Error)]
pub(crate) enum PgOrganizationPictureRefColumnsError {
    #[error("unknown organization picture type: {0}")]
    UnknownType(String),

    #[error("inconsistent organization picture columns")]
    InconsistentColumns,

    #[error("invalid organization picture object name")]
    ObjectName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization picture external URL")]
    ExternalUrl(#[source] Box<dyn std::error::Error + Send + Sync>),
}
