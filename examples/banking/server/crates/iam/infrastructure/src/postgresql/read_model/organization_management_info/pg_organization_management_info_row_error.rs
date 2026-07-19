use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationManagementInfoRowError {
    #[error("organization management info row has an invalid organization id")]
    OrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner user id")]
    OwnerUserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner username")]
    OwnerUsername(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner display name")]
    OwnerDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner picture")]
    OwnerPicture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid handle")]
    Handle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid description")]
    Description(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid website URL")]
    WebsiteUrl(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner source event id")]
    OwnerSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization management info row has an invalid owner updated event id")]
    OwnerUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
