use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgUserPublicProfileRowError {
    #[error("invalid user id")]
    UserId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid username")]
    Username(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid bio")]
    Bio(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
