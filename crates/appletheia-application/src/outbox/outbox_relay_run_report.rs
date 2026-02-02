#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutboxRelayRunReport {
    Progress {
        processed_outbox_count: super::ProcessedOutboxCount,
    },
    Idle,
    Throttled,
}
