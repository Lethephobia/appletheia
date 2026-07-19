use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgUserOrganizationJoinRequestListItemRowError {
    #[error("user organization join request list row has an invalid join request id")]
    JoinRequestId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization id")]
    OrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization handle")]
    OrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization display name")]
    OrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization picture")]
    OrganizationPicture(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an unknown status: {0}")]
    UnknownStatus(String),
    #[error("user organization join request list row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization source event id")]
    OrganizationSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization join request list row has an invalid organization updated event id")]
    OrganizationUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
