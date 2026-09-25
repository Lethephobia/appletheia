use thiserror::Error;

use crate::Retryability;
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

impl Retryability for OidcLoginFlowError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::UnitOfWorkFactory(_) | Self::UnitOfWork(_) | Self::TokenClient(_) => true,
            Self::LoginAttemptStore(error) => match error {
                OidcLoginAttemptStoreError::NotFound
                | OidcLoginAttemptStoreError::AlreadyConsumed
                | OidcLoginAttemptStoreError::Expired => false,
                OidcLoginAttemptStoreError::Backend(_) => true,
            },
            Self::ProviderMetadataSource(error) => match error {
                OidcProviderMetadataSourceError::IssuerMismatch { .. } => false,
                OidcProviderMetadataSourceError::Backend(_) => true,
            },
            Self::IdTokenVerifier(error) => match error {
                OidcIdTokenVerifierError::InvalidIdToken { .. } => false,
                OidcIdTokenVerifierError::Backend(_) => true,
            },
            Self::MissingIdToken => false,
        }
    }
}
