use thiserror::Error;

use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{
    OidcIdTokenVerifierError, OidcLoginAttemptStoreError, OidcProviderMetadataSourceError,
    OidcTokenClientError,
};

#[derive(Debug, Error)]
pub enum OidcLoginFlowError {
    #[error(transparent)]
    UnitOfWorkFactory(#[from] UnitOfWorkFactoryError),

    #[error(transparent)]
    UnitOfWork(#[from] UnitOfWorkError),

    #[error(transparent)]
    LoginAttemptStore(#[from] OidcLoginAttemptStoreError),

    #[error(transparent)]
    ProviderMetadataSource(#[from] OidcProviderMetadataSourceError),

    #[error(transparent)]
    TokenClient(#[from] OidcTokenClientError),

    #[error(transparent)]
    IdTokenVerifier(#[from] OidcIdTokenVerifierError),

    #[error("id token is missing in token response")]
    MissingIdToken,
}
