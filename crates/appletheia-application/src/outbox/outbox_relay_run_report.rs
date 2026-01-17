#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutboxRelayRunReport {
    Progress { proceeded: u32 },
    Idle,
    Throttled,
}
