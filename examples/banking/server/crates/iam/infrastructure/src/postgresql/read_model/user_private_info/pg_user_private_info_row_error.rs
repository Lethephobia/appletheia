use thiserror::Error;

/// Error returned while mapping PostgreSQL rows into user-private read models.
#[derive(Debug, Error)]
pub enum PgUserPrivateInfoRowError {
    #[error("unknown user private info status: {0}")]
    UnknownStatus(String),

    #[error("invalid user id")]
    InvalidUserId(#[source] Box<dyn std::error::Error + Send + Sync>),

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
}
