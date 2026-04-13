use super::SagaDescriptor;

/// Declares the single saga that must already exist before another saga can run.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SagaPredecessor {
    None,
    Required(&'static SagaDescriptor),
}
