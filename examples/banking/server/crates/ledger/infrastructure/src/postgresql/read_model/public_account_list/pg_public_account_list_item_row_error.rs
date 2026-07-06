use thiserror::Error;

/// Error returned while mapping PostgreSQL rows for public account list reads.
#[derive(Debug, Error)]
pub enum PgPublicAccountListItemRowError {
    #[error("unknown account owner type: {0}")]
    UnknownOwnerType(String),

    #[error("invalid user owner id")]
    InvalidUserOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization owner id")]
    InvalidOrganizationOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid username")]
    InvalidUsername(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user display name")]
    InvalidUserDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user picture")]
    InvalidUserPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("missing organization owner columns")]
    MissingOrganizationOwner,

    #[error("missing user owner columns")]
    MissingUserOwner,

    #[error("invalid source event id")]
    InvalidSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid updated event id")]
    InvalidUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization handle")]
    InvalidOrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization display name")]
    InvalidOrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization picture")]
    InvalidOrganizationPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

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

    #[error("invalid currency mint account address")]
    InvalidMintAccountAddress(#[source] Box<dyn std::error::Error + Send + Sync>),
}
