use thiserror::Error;

/// Error returned while mapping PostgreSQL rows into public user profile read models.
#[derive(Debug, Error)]
pub enum PgUserPublicProfileRowError {
    #[error("invalid user id")]
    UserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid username")]
    Username(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user display name")]
    UserDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user bio")]
    UserBio(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid user picture")]
    UserPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
