use appletheia::application::authentication::oidc::{
    OidcContinuationStoreError, OidcLoginFlowError,
};
use appletheia::domain::AggregateId;
use banking_iam_domain::UserId;
use thiserror::Error;

/// Represents errors returned while beginning an OIDC flow.
#[derive(Debug, Error)]
pub enum OidcBeginCommandHandlerError {
    #[error("oidc login flow failed")]
    OidcLoginFlow(#[from] OidcLoginFlowError),

    #[error("oidc continuation persistence failed")]
    OidcContinuationStore(#[from] OidcContinuationStoreError),

    #[error("authenticated principal contains an invalid user id")]
    InvalidAuthenticatedPrincipalUserId(#[source] <UserId as AggregateId>::Error),
}
