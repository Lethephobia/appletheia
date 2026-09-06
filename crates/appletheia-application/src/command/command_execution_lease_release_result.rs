/// Reports whether an execution lease was released.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandExecutionLeaseReleaseResult {
    Released,
    Stale,
}
