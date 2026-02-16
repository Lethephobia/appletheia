use super::ProcessedOutboxCount;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutboxRelayRunReport {
    Progress {
        processed_outbox_count: ProcessedOutboxCount,
    },
    Idle,
    Throttled,
}
