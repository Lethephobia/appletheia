use thiserror::Error;

/// Error returned while mapping PostgreSQL rows for public account list reads.
#[derive(Debug, Error)]
pub enum PgPublicAccountListItemRowError {
    #[error("unknown account owner type: {0}")]
    UnknownOwnerType(String),

    #[error("invalid account owner id")]
    InvalidOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid account id")]
    InvalidAccountId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid currency id")]
    InvalidCurrencyId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid currency symbol")]
    InvalidCurrencySymbol(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid currency name")]
    InvalidCurrencyName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid currency decimals")]
    InvalidCurrencyDecimals(#[source] Box<dyn std::error::Error + Send + Sync>),
}
