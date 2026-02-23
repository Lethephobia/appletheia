use crate::request_context::Principal;

use super::{AuthorizerError, RelationshipRequirement};

#[allow(async_fn_in_trait)]
pub trait Authorizer: Send + Sync {
    async fn authorize(
        &self,
        principal: &Principal,
        requirement: &RelationshipRequirement,
    ) -> Result<(), AuthorizerError>;
}
