use crate::messaging::Selector;

use super::{ReadModelWatchEndpointId, ReadModelWatchNotificationEnvelope};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchNotificationSelector {
    pub endpoint_id: ReadModelWatchEndpointId,
}

impl ReadModelWatchNotificationSelector {
    pub const fn new(endpoint_id: ReadModelWatchEndpointId) -> Self {
        Self { endpoint_id }
    }
}

impl Selector<ReadModelWatchNotificationEnvelope> for ReadModelWatchNotificationSelector {
    fn matches(&self, message: &ReadModelWatchNotificationEnvelope) -> bool {
        self.endpoint_id == message.notification.endpoint_id
    }
}
