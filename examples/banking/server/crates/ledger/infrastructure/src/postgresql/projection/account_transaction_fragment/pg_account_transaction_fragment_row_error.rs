use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgAccountTransactionFragmentRowError {
    #[error("unknown account transaction direction: {0}")]
    Direction(String),

    #[error("unknown account transaction kind: {0}")]
    Kind(String),

    #[error("unknown account transaction status: {0}")]
    Status(String),
}
