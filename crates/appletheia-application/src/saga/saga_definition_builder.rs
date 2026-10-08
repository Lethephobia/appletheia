use super::{SagaDefinition, SagaDefinitionBuilderError, SagaName, SagaRoute, SagaState, SagaStep};
use std::error::Error;

/// Builds a definition while carrying the state, step, and callback error types.
pub struct SagaDefinitionBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    name: SagaName,
    routes: Vec<SagaRoute<'a, S, T, E>>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaDefinitionBuilder<'a, S, T, E>
{
    pub fn new(name: SagaName) -> Self {
        Self {
            name,
            routes: Vec::new(),
        }
    }

    pub fn add_route(mut self, route: SagaRoute<'a, S, T, E>) -> Self {
        self.routes.push(route);
        self
    }

    /// Delegates route validation to the definition.
    pub fn build(self) -> Result<SagaDefinition<'a, S, T, E>, SagaDefinitionBuilderError> {
        Ok(SagaDefinition::new(self.name, self.routes)?)
    }
}
