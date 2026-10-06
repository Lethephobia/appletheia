use super::{SagaContext, SagaDefinitionBuilder, SagaRoute, SagaState, SagaStep};
use crate::command::{Command, CommandFailureEnvelopeError};
use std::error::Error;
use std::marker::PhantomData;

/// Registers a terminal command failure callback for a selected command step.
pub struct SagaFailureHandlerBuilder<
    'a,
    S: SagaState,
    T: SagaStep,
    E: Error + Send + Sync + 'static,
    C: Command,
> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
    caused_by: T,
    command: PhantomData<fn() -> C>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static, C: Command>
    SagaFailureHandlerBuilder<'a, S, T, E, C>
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
            command: PhantomData,
        }
    }

    pub fn handle<H>(self, handler: H) -> SagaDefinitionBuilder<'a, S, T, E>
    where
        E: From<CommandFailureEnvelopeError>,
        H: Fn(&mut SagaContext<'_, S, T>, &C) -> Result<(), E> + Send + Sync + 'a,
    {
        self.definition_builder
            .add_route(SagaRoute::on_command_failed::<C, H>(
                self.caused_by,
                self.step,
                handler,
            ))
    }
}
