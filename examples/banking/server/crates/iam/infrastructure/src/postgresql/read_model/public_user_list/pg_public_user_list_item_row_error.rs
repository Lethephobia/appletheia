use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgPublicUserListItemRowError {
    #[error("public user list item row has an invalid user id")]
    UserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("public user list item row has an invalid username")]
    Username(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("public user list item row has an invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("public user list item row has an invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("public user list item row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("public user list item row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
