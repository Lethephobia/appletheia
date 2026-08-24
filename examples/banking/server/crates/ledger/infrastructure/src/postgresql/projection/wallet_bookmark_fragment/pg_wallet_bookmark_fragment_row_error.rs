use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum PgWalletBookmarkFragmentRowError {
    #[error("unknown wallet bookmark owner type: {0}")]
    OwnerType(String),
}
