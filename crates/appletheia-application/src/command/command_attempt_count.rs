use serde::{Deserialize, Serialize};

use super::CommandAttemptCountError;

/// Counts handler executions for one command message.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandAttemptCount(u32);

impl CommandAttemptCount {
    pub const fn first() -> Self {
        Self(1)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

impl TryFrom<i64> for CommandAttemptCount {
    type Error = CommandAttemptCountError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let converted = u32::try_from(value).map_err(|_| CommandAttemptCountError::OutOfRange)?;
        if converted == 0 {
            return Err(CommandAttemptCountError::Zero);
        }
        Ok(Self(converted))
    }
}
