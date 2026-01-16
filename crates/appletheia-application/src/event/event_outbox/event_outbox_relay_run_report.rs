#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventOutboxRelayRunReport {
    Progress { proceeded: u32 },
    Idle,
    Throttled,
}
