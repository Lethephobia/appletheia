use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::SagaName;

/// Describes a saga's identity and subscribed events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SagaDescriptor {
    pub name: SagaName,
    pub subscription: Subscription<'static, EventSelector>,
}

impl SagaDescriptor {
    /// Creates a new saga descriptor.
    pub const fn new(name: SagaName, subscription: Subscription<'static, EventSelector>) -> Self {
        Self { name, subscription }
    }
}
