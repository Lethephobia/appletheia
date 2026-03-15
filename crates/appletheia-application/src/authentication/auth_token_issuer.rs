use super::{AuthTokenIssueRequest, AuthTokenIssueResult, AuthTokenIssuerError};

#[allow(async_fn_in_trait)]
pub trait AuthTokenIssuer: Send + Sync {
    async fn issue(
        &self,
        request: AuthTokenIssueRequest,
    ) -> Result<AuthTokenIssueResult, AuthTokenIssuerError>;
}
