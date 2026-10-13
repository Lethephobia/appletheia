use crate::event::EventFeedBatchSize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProjectionCatchUpConfig {
    pub batch_size: EventFeedBatchSize,
}
