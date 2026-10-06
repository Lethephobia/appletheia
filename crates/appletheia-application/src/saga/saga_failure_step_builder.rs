use super::{SagaDefinitionBuilder, SagaFailureHandlerBuilder, SagaState, SagaStep};
use crate::command::Command;
use std::error::Error;

/// Selects the input condition for a failure route.
pub struct SagaFailureStepBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaFailureStepBuilder<'a, S, T, E>
{
    pub(crate) fn new(definition_builder: SagaDefinitionBuilder<'a, S, T, E>, step: T) -> Self {
        Self {
            definition_builder,
            step,
        }
    }

    pub fn on<C: Command>(self, caused_by: T) -> SagaFailureHandlerBuilder<'a, S, T, E, C> {
        SagaFailureHandlerBuilder::new(self.definition_builder, self.step, caused_by)
    }
}
