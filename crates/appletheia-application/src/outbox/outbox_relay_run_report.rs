#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutboxRelayRunReport {
    Progress { proceeded_outbox_count: u32 },
    Idle,
    Throttled,
}
