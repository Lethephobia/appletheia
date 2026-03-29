use super::{SagaDescriptor, SagaState};

/// Defines the stable descriptor and state type for a saga.
pub trait SagaSpec {
    type State: SagaState;

    const DESCRIPTOR: SagaDescriptor;
}
