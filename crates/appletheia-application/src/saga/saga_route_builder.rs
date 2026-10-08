use super::{SagaCommandFailureRouteHandlerBuilder, SagaEventRouteBuilder, SagaStep};
use crate::command::Command;
use appletheia_domain::{Aggregate, EventName};

/// Selects the input for commands dispatched under the given step.
pub struct SagaRouteBuilder<T: SagaStep> {
    step: T,
}

impl<T: SagaStep> SagaRouteBuilder<T> {
    pub fn new(step: T) -> Self {
        Self { step }
    }

    pub fn on<A: Aggregate>(self, event_name: EventName) -> SagaEventRouteBuilder<T, A> {
        SagaEventRouteBuilder::new(self.step, event_name)
    }

    pub fn on_command_failed<C: Command>(
        self,
        caused_by: T,
    ) -> SagaCommandFailureRouteHandlerBuilder<T, C> {
        SagaCommandFailureRouteHandlerBuilder::new(self.step, caused_by)
    }
}
