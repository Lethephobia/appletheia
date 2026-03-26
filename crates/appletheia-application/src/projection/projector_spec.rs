use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::ProjectorName;

/// Defines the stable identity and subscription for a projector.
pub trait ProjectorSpec {
    const NAME: ProjectorName;
    const SUBSCRIPTION: Subscription<'static, EventSelector>;
}
