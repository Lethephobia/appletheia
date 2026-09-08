use super::{SagaDefinitionBuilder, SagaEventHandlerBuilder, SagaState, SagaStep};
use crate::event::EventSelector;
use appletheia_domain::{Aggregate, EventName};
use std::error::Error;

/// Selects the input condition for a continuation event route.
pub struct SagaStepBuilder<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> SagaStepBuilder<'a, S, T, E> {
    pub(crate) fn new(definition_builder: SagaDefinitionBuilder<'a, S, T, E>, step: T) -> Self {
        Self {
            definition_builder,
            step,
        }
    }

    pub fn on<A: Aggregate>(
        self,
        caused_by: T,
        event_name: EventName,
    ) -> SagaEventHandlerBuilder<'a, S, T, E, A> {
        SagaEventHandlerBuilder::new(
            self.definition_builder,
            self.step,
            EventSelector::new::<A>(event_name),
            caused_by,
        )
    }
}
