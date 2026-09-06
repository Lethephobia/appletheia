use super::SagaDescriptor;

/// Defines the stable descriptor for a saga.
pub trait SagaSpec {
    const DESCRIPTOR: SagaDescriptor;
}
