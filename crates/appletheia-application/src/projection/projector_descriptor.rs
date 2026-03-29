use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::ProjectorName;

/// Describes a projector's identity and subscribed events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProjectorDescriptor {
    pub name: ProjectorName,
    pub subscription: Subscription<'static, EventSelector>,
}

impl ProjectorDescriptor {
    /// Creates a new projector descriptor.
    pub const fn new(
        name: ProjectorName,
        subscription: Subscription<'static, EventSelector>,
    ) -> Self {
        Self { name, subscription }
    }
}
