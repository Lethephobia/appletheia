use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgOrganizationJoinRequestFragmentRowError {
    #[error("unknown organization join request status: {0}")]
    Status(String),
}
