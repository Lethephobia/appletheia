use thiserror::Error;

use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{
    AuthTokenExchangeCodeHasherError, AuthTokenExchangeCodeStoreError,
    AuthTokenExchangeGrantCipherError, AuthTokenIssuerError,
};

/// Errors returned while exchanging a one-time code for tokens.
#[derive(Debug, Error)]
pub enum AuthTokenExchangerError {
    #[error(transparent)]
    Store(#[from] AuthTokenExchangeCodeStoreError),

    #[error(transparent)]
    Hasher(#[from] AuthTokenExchangeCodeHasherError),

    #[error(transparent)]
    GrantCipher(#[from] AuthTokenExchangeGrantCipherError),

    #[error(transparent)]
    AuthTokenIssuer(#[from] AuthTokenIssuerError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error("code verifier is required")]
    MissingCodeVerifier,

    #[error("code verifier is not allowed")]
    UnexpectedCodeVerifier,

    #[error("code verifier does not match the stored challenge")]
    InvalidCodeVerifier,
}
