use super::{SagaFailureHandlerBuilder, SagaName, SagaRoute, SagaState, SagaStep};
use crate::command::Command;
use std::error::Error;

/// Selects the input condition for a failure route.
pub struct SagaFailureStepBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    name: SagaName,
    routes: Vec<SagaRoute<'a, S, T, E>>,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaFailureStepBuilder<'a, S, T, E>
{
    pub(crate) fn new(name: SagaName, routes: Vec<SagaRoute<'a, S, T, E>>, step: T) -> Self {
        Self { name, routes, step }
    }

    pub fn on<C: Command>(self, caused_by: T) -> SagaFailureHandlerBuilder<'a, S, T, E, C> {
        SagaFailureHandlerBuilder::new(self.name, self.routes, self.step, caused_by)
    }
}
