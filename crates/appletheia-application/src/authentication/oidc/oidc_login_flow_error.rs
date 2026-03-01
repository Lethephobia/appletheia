use thiserror::Error;

use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

use super::{
    OidcIdTokenVerifyError, OidcLoginAttemptStoreError, OidcProviderMetadataSourceError,
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
    IdTokenVerify(#[from] OidcIdTokenVerifyError),

    #[error("id token is missing in token response")]
    MissingIdToken,
}
