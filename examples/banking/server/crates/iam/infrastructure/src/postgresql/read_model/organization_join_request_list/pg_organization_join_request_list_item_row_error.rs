use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationJoinRequestListItemRowError {
    #[error("organization join request list row has an invalid join request id")]
    JoinRequestId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization join request list row has an invalid requester user id")]
    RequesterUserId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization join request list row has an unknown status: {0}")]
    UnknownStatus(String),
    #[error("organization join request list row has an invalid requester username")]
    RequesterUsername(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization join request list row has an invalid requester display name")]
    RequesterDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization join request list row has an invalid requester picture")]
    RequesterPicture(#[source] Box<dyn std::error::Error + Send + Sync>),
}
