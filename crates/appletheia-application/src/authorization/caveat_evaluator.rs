use std::error::Error;

use crate::request_context::Principal;

use super::{AuthorizationRequest, Caveat};

#[allow(async_fn_in_trait)]
pub trait CaveatEvaluator: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    async fn evaluate(
        &self,
        principal: &Principal,
        request: &AuthorizationRequest,
        caveat: &Caveat,
    ) -> Result<bool, Self::Error>;
}

