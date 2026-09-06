use std::num::NonZeroU32;

/// Limits handler executions for one asynchronous command message.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CommandExecutionMaxAttempts(NonZeroU32);

impl CommandExecutionMaxAttempts {
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> NonZeroU32 {
        self.0
    }
}

impl Default for CommandExecutionMaxAttempts {
    fn default() -> Self {
        Self(NonZeroU32::MIN.saturating_add(2))
    }
}
