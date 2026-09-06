use super::{CommandExecutionLeaseDuration, CommandExecutionMaxAttempts};

/// Configures handler execution retries independently from outbox publication retries.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandExecutionRetryPolicy {
    pub max_attempts: CommandExecutionMaxAttempts,
    pub lease_duration: CommandExecutionLeaseDuration,
}
