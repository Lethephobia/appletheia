use super::ProcessedEventCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProjectorRebuildReport {
    pub processed_event_count: ProcessedEventCount,
}
