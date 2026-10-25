use crate::messaging::ConsumerGroup;

use super::ReadModelWatchEndpointPageSize;

#[derive(Clone, Debug)]
pub struct ReadModelWatchNotificationWorkerConfig {
    pub consumer_group: ConsumerGroup,
    pub endpoint_page_size: ReadModelWatchEndpointPageSize,
}
