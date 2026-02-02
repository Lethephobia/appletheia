use crate::event::EventFeedBatchSize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProjectorRebuilderConfig {
    pub batch_size: EventFeedBatchSize,
}
