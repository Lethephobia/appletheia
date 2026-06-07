use thiserror::Error;

/// Error returned while mapping PostgreSQL rows into user-private read models.
#[derive(Debug, Error)]
pub enum PgUserPrivateInfoRowError {
    #[error("unknown user private info status: {0}")]
    UnknownStatus(String),

    #[error("invalid user id")]
    InvalidUserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization id")]
    InvalidOrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("missing organization")]
    MissingOrganization,

    #[error("invalid organization handle")]
    InvalidOrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization display name")]
    InvalidOrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization picture")]
    InvalidOrganizationPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid organization roles")]
    InvalidOrganizationRoles(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid username")]
    InvalidUsername(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user display name")]
    InvalidUserDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user bio")]
    InvalidUserBio(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user picture")]
    InvalidUserPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user identity provider")]
    InvalidUserIdentityProvider(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user identity subject")]
    InvalidUserIdentitySubject(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid email")]
    InvalidEmail(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid source event id")]
    InvalidSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid updated event id")]
    InvalidUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
