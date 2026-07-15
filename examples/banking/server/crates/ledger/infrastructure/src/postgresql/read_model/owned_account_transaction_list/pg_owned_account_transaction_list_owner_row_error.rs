use thiserror::Error;

/// Error returned while mapping PostgreSQL rows into owned account transaction list owners.
#[derive(Debug, Error)]
pub enum PgOwnedAccountTransactionListOwnerRowError {
    #[error("unknown account owner type: {0}")]
    UnknownOwnerType(String),

    #[error("organization owner row is missing")]
    MissingOrganizationOwner,

    #[error("user owner row is missing")]
    MissingUserOwner,

    #[error("invalid source event id")]
    InvalidSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid updated event id")]
    InvalidUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

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

    #[error("invalid organization handle")]
    InvalidOrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization display name")]
    InvalidOrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization picture")]
    InvalidOrganizationPicture(#[source] Box<dyn std::error::Error + Send + Sync>),
}
