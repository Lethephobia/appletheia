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

    fn on_event(
        &self,
        instance: &mut SagaInstance<Self::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error>;
}
