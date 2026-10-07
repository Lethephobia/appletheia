use super::{
    SagaDefinition, SagaDefinitionBuilderError, SagaFailureStepBuilder, SagaName, SagaRoute,
    SagaStartStepBuilder, SagaState, SagaStep, SagaStepBuilder,
};
use std::error::Error;

/// Builds a definition while carrying the state, step, and callback error types.
pub struct SagaDefinitionBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    pub(super) name: SagaName,
    pub(super) routes: Vec<SagaRoute<'a, S, T, E>>,
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

    pub fn add_start_step(self, step: T) -> SagaStartStepBuilder<'a, S, T, E> {
        SagaStartStepBuilder::new(self.name, self.routes, step)
    }

    pub fn add_step(self, step: T) -> SagaStepBuilder<'a, S, T, E> {
        SagaStepBuilder::new(self.name, self.routes, step)
    }

    pub fn add_failure_step(self, step: T) -> SagaFailureStepBuilder<'a, S, T, E> {
        SagaFailureStepBuilder::new(self.name, self.routes, step)
    }

    /// Delegates route validation to the definition.
    pub fn build(self) -> Result<SagaDefinition<'a, S, T, E>, SagaDefinitionBuilderError> {
        Ok(SagaDefinition::new(self.name, self.routes)?)
    }
}
