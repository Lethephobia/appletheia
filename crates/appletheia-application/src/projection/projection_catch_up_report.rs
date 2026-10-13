use super::ProcessedEventCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProjectionCatchUpReport {
    pub processed_event_count: ProcessedEventCount,
}
