use crate::event::EventEnvelope;

use super::{SagaName, SagaOutcome, SagaState};

pub trait SagaDefinition: Send + Sync {
    type State: SagaState;

    const NAME: SagaName;

    fn on_event(&self, state: &mut Option<Self::State>, event: &EventEnvelope) -> SagaOutcome;
}
