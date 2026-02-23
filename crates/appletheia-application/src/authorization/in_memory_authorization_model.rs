use std::collections::HashMap;

use crate::event::AggregateTypeOwned;

use super::{AuthorizationModel, AuthorizationTypeDefinition};

#[derive(Clone, Debug, Default)]
pub struct InMemoryAuthorizationModel {
    types: HashMap<AggregateTypeOwned, AuthorizationTypeDefinition>,
}

impl InMemoryAuthorizationModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_type(
        &mut self,
        aggregate_type: AggregateTypeOwned,
        type_definition: AuthorizationTypeDefinition,
    ) {
        self.types.insert(aggregate_type, type_definition);
    }
}

impl AuthorizationModel for InMemoryAuthorizationModel {
    fn type_definition_for(
        &self,
        aggregate_type: &AggregateTypeOwned,
    ) -> Option<&AuthorizationTypeDefinition> {
        self.types.get(aggregate_type)
    }
}
