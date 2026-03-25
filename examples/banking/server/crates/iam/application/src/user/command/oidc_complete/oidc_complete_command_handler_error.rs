use appletheia::application::authentication::oidc::{
    OidcContinuationStoreError, OidcLoginFlowError,
};
use appletheia::application::authentication::{
    AuthTokenClaimsError, AuthTokenExchangeCodeIssuerError, AuthTokenIssuerError,
};
use appletheia::application::authorization::AggregateRefError;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::{UniqueValueError, UniqueValuePartError};
use banking_iam_domain::{
    EmailError, User, UserError, UserIdentityProviderError, UserIdentitySubjectError,
};
use thiserror::Error;

/// Represents errors returned while completing an OIDC flow.
#[derive(Debug, Error)]
pub enum OidcCompleteCommandHandlerError {
    #[error("oidc login flow failed")]
    OidcLoginFlow(#[from] OidcLoginFlowError),

    #[error("oidc continuation persistence failed")]
    OidcContinuationStore(#[from] OidcContinuationStoreError),

    #[error("user repository failed")]
    UserRepository(#[from] RepositoryError<User>),

    #[error("user aggregate failed")]
    User(#[from] UserError),

    #[error("user identity provider is invalid")]
    UserIdentityProvider(#[from] UserIdentityProviderError),

    #[error("user identity subject is invalid")]
    UserIdentitySubject(#[from] UserIdentitySubjectError),

    #[error("email is invalid")]
    Email(#[from] EmailError),

    #[error("unique value part is invalid")]
    UniqueValuePart(#[from] UniqueValuePartError),

    #[error("unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),

    #[error("auth token issue failed")]
    AuthTokenIssuer(#[from] AuthTokenIssuerError),

    #[error("auth token claims are invalid")]
    AuthTokenClaims(#[from] AuthTokenClaimsError),

    #[error("auth token exchange code issue failed")]
    AuthTokenExchangeCodeIssuer(#[from] AuthTokenExchangeCodeIssuerError),

    #[error("aggregate ref is invalid")]
    AggregateRef(#[from] AggregateRefError),
}
