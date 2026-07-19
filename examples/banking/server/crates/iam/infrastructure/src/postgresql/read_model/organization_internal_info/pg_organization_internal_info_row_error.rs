use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationInternalInfoRowError {
    #[error("organization internal info row has an invalid organization id")]
    OrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid handle")]
    Handle(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid description")]
    Description(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid website URL")]
    WebsiteUrl(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization internal info row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
