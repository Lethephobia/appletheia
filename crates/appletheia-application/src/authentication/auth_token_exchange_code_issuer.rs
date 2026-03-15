use crate::unit_of_work::UnitOfWork;

use super::{
    AuthTokenExchangeCodeIssueRequest, AuthTokenExchangeCodeIssueResult,
    AuthTokenExchangeCodeIssuerError,
};

/// Issues one-time exchange codes that can later be redeemed for auth tokens.
#[allow(async_fn_in_trait)]
pub trait AuthTokenExchangeCodeIssuer: Send + Sync {
    /// The unit of work type used by the issuer.
    type Uow: UnitOfWork;

    /// Issues a new exchange code for the provided request.
    async fn issue(
        &self,
        uow: &mut Self::Uow,
        request: AuthTokenExchangeCodeIssueRequest,
    ) -> Result<AuthTokenExchangeCodeIssueResult, AuthTokenExchangeCodeIssuerError>;
}
