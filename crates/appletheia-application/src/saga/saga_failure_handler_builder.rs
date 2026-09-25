use super::{SagaContext, SagaDefinitionBuilder, SagaRoute, SagaState, SagaStep};
use crate::command::CommandFailureEnvelope;
use std::error::Error;

/// Registers a terminal command failure callback for a selected command step.
pub struct SagaFailureHandlerBuilder<
    'a,
    S: SagaState,
    T: SagaStep,
    E: Error + Send + Sync + 'static,
> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
    caused_by: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaFailureHandlerBuilder<'a, S, T, E>
{
    pub(crate) fn new(
        definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
        step: T,
        caused_by: T,
    ) -> Self {
        Self {
            definition_builder,
            step,
            caused_by,
        }
    }

    pub fn handle<H>(self, handler: H) -> SagaDefinitionBuilder<'a, S, T, E>
    where
        H: Fn(&mut SagaContext<'_, S, T>, &CommandFailureEnvelope) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        self.definition_builder
            .add_route(SagaRoute::on_command_failed(
                self.caused_by,
                self.step,
                handler,
            ))
    }
}
