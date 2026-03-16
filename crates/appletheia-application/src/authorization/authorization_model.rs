use crate::event::AggregateTypeOwned;

use super::{AuthorizationModelError, AuthorizationTypeDefinition};

#[allow(async_fn_in_trait)]
pub trait AuthorizationModel: Send + Sync {
    async fn type_definition_for(
        &self,
        aggregate_type: &AggregateTypeOwned,
    ) -> Result<Option<AuthorizationTypeDefinition>, AuthorizationModelError>;
}
