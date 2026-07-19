use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationMemberListItemRowError {
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

    #[error("organization member list row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid member source event id")]
    MemberSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("organization member list row has an invalid member updated event id")]
    MemberUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
