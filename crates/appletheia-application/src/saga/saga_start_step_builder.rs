use super::{SagaName, SagaRoute, SagaStartEventHandlerBuilder, SagaState, SagaStep};
use appletheia_domain::{Aggregate, EventName};
use std::error::Error;

/// Selects the input condition for a start route.
pub struct SagaStartStepBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    name: SagaName,
    routes: Vec<SagaRoute<'a, S, T, E>>,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaStartStepBuilder<'a, S, T, E>
{
    pub(crate) fn new(name: SagaName, routes: Vec<SagaRoute<'a, S, T, E>>, step: T) -> Self {
        Self { name, routes, step }
    }

    pub fn on<A: Aggregate>(
        self,
        event_name: EventName,
    ) -> SagaStartEventHandlerBuilder<'a, S, T, E, A> {
        SagaStartEventHandlerBuilder::new(self.name, self.routes, self.step, event_name)
    }
}
