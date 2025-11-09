use std::{fmt, fmt::Display};

use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CommandId(Uuid);

impl CommandId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for CommandId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CommandId> for Uuid {
    fn from(value: CommandId) -> Self {
        value.0
    }
}

impl Display for CommandId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
