use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgWalletBookmarkListItemRowError {
    #[error("wallet bookmark list item row has an invalid wallet bookmark id")]
    InvalidWalletBookmarkId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an unknown owner type: {0}")]
    UnknownOwnerType(String),

    #[error("wallet bookmark list item row has an invalid user owner id")]
    InvalidUserOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid organization owner id")]
    InvalidOrganizationOwnerId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid display name")]
    InvalidDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid description")]
    InvalidDescription(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid token account owner address")]
    InvalidTokenAccountOwnerAddress(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid source event id")]
    InvalidSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("wallet bookmark list item row has an invalid updated event id")]
    InvalidUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
