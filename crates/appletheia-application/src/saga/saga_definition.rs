use serde::{Serialize, de::DeserializeOwned};

use crate::event::AppEvent;

use super::{SagaName, SagaOutcome};

pub trait SagaDefinition: Send + Sync {
    type State: Serialize + DeserializeOwned + Send + Sync;

    const NAME: SagaName;

    fn initial_state(&self, first_event: &AppEvent) -> Self::State;

    fn on_event(&self, state: &mut Self::State, event: &AppEvent) -> SagaOutcome;
}
