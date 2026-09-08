use super::{SagaDefinitionBuilder, SagaStartEventHandlerBuilder, SagaState, SagaStep};
use crate::event::EventSelector;
use appletheia_domain::{Aggregate, EventName};
use std::error::Error;

/// Selects the input condition for a start route.
pub struct SagaStartStepBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static>
    SagaStartStepBuilder<'a, S, T, E>
{
    pub(crate) fn new(definition_builder: SagaDefinitionBuilder<'a, S, T, E>, step: T) -> Self {
        Self {
            definition_builder,
            step,
        }
    }

    pub fn on<A: Aggregate>(
        self,
        event_name: EventName,
    ) -> SagaStartEventHandlerBuilder<'a, S, T, E, A> {
        SagaStartEventHandlerBuilder::new(
            self.definition_builder,
            self.step,
            EventSelector::new::<A>(event_name),
        )
    }
}
