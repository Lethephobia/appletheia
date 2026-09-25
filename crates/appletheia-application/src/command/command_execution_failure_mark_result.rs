use super::CommandFailedAt;

/// Reports whether a command execution was marked as failed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandExecutionFailureMarkResult {
    Marked { failed_at: CommandFailedAt },
    Stale,
}
