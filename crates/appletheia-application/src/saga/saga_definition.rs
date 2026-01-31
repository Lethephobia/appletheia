use std::error::Error;

use crate::event::EventEnvelope;

use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::{SagaInstance, SagaName, SagaState};

pub trait SagaDefinition: Send + Sync {
    type State: SagaState;
    type Error: Error + Send + Sync + 'static;

    const NAME: SagaName;
    const SUBSCRIPTION: Subscription<'static, EventSelector>;

    fn matches(&self, event: &EventEnvelope) -> bool {
        match Self::SUBSCRIPTION {
            Subscription::All => true,
            Subscription::Only(selectors) => selectors.iter().any(|selector| selector.matches(event)),
        }
    }

    fn on_event(
        &self,
        instance: &mut SagaInstance<Self::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error>;
}
