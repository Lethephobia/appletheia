use crate::event::AggregateTypeOwned;

use super::AuthorizationTypeDefinition;

pub trait AuthorizationModel: Send + Sync {
    fn type_definition_for(
        &self,
        aggregate_type: &AggregateTypeOwned,
    ) -> Option<&AuthorizationTypeDefinition>;
}
