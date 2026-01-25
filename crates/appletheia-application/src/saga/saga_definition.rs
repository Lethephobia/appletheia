use std::error::Error;

use crate::event::EventEnvelope;

use crate::event::EventSelector;

use super::{SagaInstance, SagaName, SagaState};

pub trait SagaDefinition: Send + Sync {
    type State: SagaState;
    type Error: Error + Send + Sync + 'static;

    const NAME: SagaName;
    const EVENTS: &'static [EventSelector] = &[];

    fn matches(&self, event: &EventEnvelope) -> bool {
        let events = Self::EVENTS;
        if events.is_empty() {
            return true;
        }
        events.iter().any(|selector| selector.matches(event))
    }

    fn on_event(
        &self,
        instance: &mut SagaInstance<Self::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error>;
}
