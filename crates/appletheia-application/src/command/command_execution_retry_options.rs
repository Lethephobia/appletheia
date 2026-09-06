use super::CommandExecutionMaxAttempts;

/// Configures retries of asynchronous command-handler execution.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandExecutionRetryOptions {
    pub max_attempts: CommandExecutionMaxAttempts,
}
