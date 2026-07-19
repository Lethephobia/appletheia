use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOrganizationInvitationListItemRowError {
    #[error("organization invitation list row has an invalid invitation id")]
    InvitationId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid invitee user id")]
    InviteeUserId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has invalid roles")]
    Roles(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid issuer")]
    Issuer,
    #[error("organization invitation list row has an invalid issuer user id")]
    IssuerUserId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an unknown status: {0}")]
    UnknownStatus(String),
    #[error("organization invitation list row has an invalid invitee username")]
    InviteeUsername(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid invitee display name")]
    InviteeDisplayName(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid invitee picture")]
    InviteePicture(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid source event id")]
    SourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid updated event id")]
    UpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid invitee source event id")]
    InviteeSourceEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("organization invitation list row has an invalid invitee updated event id")]
    InviteeUpdatedEventId(#[source] Box<dyn std::error::Error + Send + Sync>),
}
