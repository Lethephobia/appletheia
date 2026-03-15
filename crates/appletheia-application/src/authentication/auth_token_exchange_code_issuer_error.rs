use thiserror::Error;

use crate::unit_of_work::UnitOfWorkError;

use super::{
    AuthTokenExchangeCodeHasherError, AuthTokenExchangeCodeStoreError,
    AuthTokenExchangeGrantCipherError,
};

/// Errors returned while issuing auth token exchange codes.
#[derive(Debug, Error)]
pub enum AuthTokenExchangeCodeIssuerError {
    #[error("code challenge is required")]
    MissingCodeChallenge,

    #[error("code challenge is not allowed")]
    UnexpectedCodeChallenge,

    #[error(transparent)]
    Hasher(#[from] AuthTokenExchangeCodeHasherError),

    #[error(transparent)]
    Store(#[from] AuthTokenExchangeCodeStoreError),

    #[error(transparent)]
    GrantCipher(#[from] AuthTokenExchangeGrantCipherError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),
}
