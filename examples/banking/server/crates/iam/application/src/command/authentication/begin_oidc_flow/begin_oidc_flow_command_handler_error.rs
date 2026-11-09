use appletheia::application::Retryability;
use appletheia::application::authentication::oidc::{
    OidcContinuationStoreError, OidcLoginFlowError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BeginOidcFlowCommandHandlerError {
    #[error("oidc completion redirect URI is not allowed")]
    CompletionRedirectUriNotAllowed,

    #[error("oidc login flow failed")]
    OidcLoginFlow(#[from] OidcLoginFlowError),

    #[error("oidc continuation persistence failed")]
    OidcContinuationStore(#[from] OidcContinuationStoreError),
}

impl Retryability for BeginOidcFlowCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CompletionRedirectUriNotAllowed => false,
            Self::OidcLoginFlow(error) => error.is_retryable(),
            Self::OidcContinuationStore(error) => error.is_retryable(),
        }
    }
}
