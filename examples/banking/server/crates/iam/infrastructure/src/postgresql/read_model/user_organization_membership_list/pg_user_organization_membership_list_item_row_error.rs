use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgUserOrganizationMembershipListItemRowError {
    #[error("user organization membership list row has an invalid organization membership id")]
    OrganizationMembershipId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization id")]
    OrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization handle")]
    OrganizationHandle(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization display name")]
    OrganizationDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization picture")]
    OrganizationPicture(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has invalid roles")]
    Roles(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization source event id")]
    OrganizationSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("user organization membership list row has an invalid organization updated event id")]
    OrganizationUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
