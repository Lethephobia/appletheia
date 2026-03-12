use crate::request_context::Principal;

use super::{AuthorizationPlan, AuthorizerError};

#[allow(async_fn_in_trait)]
pub trait Authorizer: Send + Sync {
    async fn authorize(
        &self,
        principal: &Principal,
        authorization_plan: &AuthorizationPlan,
    ) -> Result<(), AuthorizerError>;
}
