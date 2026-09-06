use super::CommandAttemptCount;

/// Reports whether a delivered command acquired its execution lease.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandExecutionLeaseAcquisitionResult {
    Acquired { attempt_count: CommandAttemptCount },
    InProgress,
    Succeeded,
    Failed,
}
