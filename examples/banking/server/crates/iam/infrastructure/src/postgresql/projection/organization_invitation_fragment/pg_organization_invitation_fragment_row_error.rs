use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgOrganizationInvitationFragmentRowError {
    #[error(
        "invalid organization invitation issuer type {issuer_type} with user ID present: {user_id_present}"
    )]
    Issuer {
        issuer_type: String,
        user_id_present: bool,
    },

    #[error("unknown organization invitation status: {0}")]
    Status(String),
}
