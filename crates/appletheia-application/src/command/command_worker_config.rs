use super::{CommandExecutionLeaseDuration, CommandExecutionRetryOptions};

/// Configures asynchronous command execution by a command worker.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandWorkerConfig {
    pub lease_duration: CommandExecutionLeaseDuration,
    pub retry_options: CommandExecutionRetryOptions,
}
