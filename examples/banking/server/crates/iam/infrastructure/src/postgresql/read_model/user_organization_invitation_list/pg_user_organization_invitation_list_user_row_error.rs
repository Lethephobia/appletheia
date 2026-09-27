use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgUserOrganizationInvitationListUserRowError {
    #[error("user organization invitation list user row has an invalid user id")]
    UserId(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("user organization invitation list user row has an invalid username")]
    Username(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("user organization invitation list user row has an invalid display name")]
    DisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("user organization invitation list user row has an invalid picture")]
    Picture(#[source] Box<dyn std::error::Error + Send + Sync>),
}
