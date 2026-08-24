use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgAccountFragmentRowError {
    #[error("unknown account fragment status: {0}")]
    Status(String),

    #[error("unknown account fragment owner type: {0}")]
    OwnerType(String),
}
