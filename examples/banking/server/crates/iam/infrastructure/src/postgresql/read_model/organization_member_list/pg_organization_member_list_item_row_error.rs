use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationMemberListItemRowError {
    #[error("organization member list row has an invalid organization id")]
    OrganizationId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid user id")]
    UserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid username")]
    Username(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has invalid roles")]
    Roles(#[source] Box<dyn std::error::Error + Send + Sync>),
}
