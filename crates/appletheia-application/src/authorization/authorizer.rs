use crate::request_context::Principal;

use super::{AuthorizationRequest, AuthorizerError};

#[allow(async_fn_in_trait)]
pub trait Authorizer: Send + Sync {
    async fn authorize(
        &self,
        principal: &Principal,
        request: AuthorizationRequest,
    ) -> Result<(), AuthorizerError>;
}
