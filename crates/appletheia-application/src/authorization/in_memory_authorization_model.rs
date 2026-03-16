use std::collections::HashMap;
use std::sync::Arc;

use crate::event::AggregateTypeOwned;

use super::{AuthorizationModel, AuthorizationModelError, AuthorizationTypeDefinition};

#[derive(Clone, Debug, Default)]
pub struct InMemoryAuthorizationModel {
    types: HashMap<AggregateTypeOwned, Arc<AuthorizationTypeDefinition>>,
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
        self.types.insert(aggregate_type, Arc::new(type_definition));
    }

    pub fn define_shared_type(
        &mut self,
        aggregate_type: AggregateTypeOwned,
        type_definition: Arc<AuthorizationTypeDefinition>,
    ) {
        self.types.insert(aggregate_type, type_definition);
    }
}

impl AuthorizationModel for InMemoryAuthorizationModel {
    async fn type_definition_for(
        &self,
        aggregate_type: &AggregateTypeOwned,
    ) -> Result<Option<AuthorizationTypeDefinition>, AuthorizationModelError> {
        Ok(self
            .types
            .get(aggregate_type)
            .map(|definition| (**definition).clone()))
    }
}
