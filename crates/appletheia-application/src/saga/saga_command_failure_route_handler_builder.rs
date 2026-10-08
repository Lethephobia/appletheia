use std::{error::Error, marker::PhantomData};

use super::{SagaContext, SagaRoute, SagaState, SagaStep};
use crate::command::{Command, CommandFailureEnvelopeError};

pub struct SagaCommandFailureRouteHandlerBuilder<T: SagaStep, C: Command> {
    step: T,
    caused_by: T,
    command: PhantomData<fn() -> C>,
}

impl<T: SagaStep, C: Command> SagaCommandFailureRouteHandlerBuilder<T, C> {
    pub(crate) fn new(step: T, caused_by: T) -> Self {
        Self {
            step,
            caused_by,
            command: PhantomData,
        }
    }

    pub fn handle<'a, S, E, H>(self, handler: H) -> SagaRoute<'a, S, T, E>
    where
        S: SagaState,
        E: Error + From<CommandFailureEnvelopeError> + Send + Sync + 'static,
        H: Fn(&mut SagaContext<'_, S, T>, &C) -> Result<(), E> + Send + Sync + 'a,
    {
        SagaRoute::on_command_failed::<C, H>(self.caused_by, self.step, handler)
    }
}
