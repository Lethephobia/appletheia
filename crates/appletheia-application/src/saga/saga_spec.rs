use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::{SagaName, SagaState};

/// Defines the stable identity, state type, and subscription for a saga.
pub trait SagaSpec {
    type State: SagaState;

    const NAME: SagaName;
    const SUBSCRIPTION: Subscription<'static, EventSelector>;
}
