use thiserror::Error;

/// Error returned while mapping PostgreSQL rows into owned account transaction list read models.
#[derive(Debug, Error)]
pub enum PgOwnedAccountTransactionListItemRowError {
    #[error("unknown account owner type: {0}")]
    UnknownOwnerType(String),

    #[error("unknown transaction direction: {0}")]
    UnknownDirection(String),

    #[error("unknown transaction kind: {0}")]
    UnknownKind(String),

    #[error("transfer transaction is missing transfer attributes")]
    MissingTransferAttributes,

    #[error("transfer transaction is missing counterparty account owner")]
    MissingCounterpartyAccountOwner,

    #[error("organization counterparty account owner is missing handle or display name")]
    MissingCounterpartyAccountOwnerOrganization,

    #[error("non-transfer transaction has transfer attributes")]
    UnexpectedTransferAttributes,

    #[error("unknown transaction status: {0}")]
    UnknownStatus(String),

    #[error("invalid event id")]
    InvalidEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid transfer id")]
    InvalidTransferId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid account owner id")]
    InvalidOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user owner id")]
    InvalidUserOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization owner id")]
    InvalidOrganizationOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid username")]
    InvalidUsername(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user display name")]
    InvalidUserDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization handle")]
    InvalidOrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization display name")]
    InvalidOrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

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

    #[error("invalid currency amount")]
    InvalidCurrencyAmount(#[source] std::num::ParseIntError),
}
