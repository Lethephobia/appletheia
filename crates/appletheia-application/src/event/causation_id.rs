use crate::command::CommandId;
use std::{fmt, fmt::Display};

use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CausationId(Uuid);

impl CausationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for CausationId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CausationId> for Uuid {
    fn from(value: CausationId) -> Self {
        value.0
    }
}

impl Display for CausationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CommandId> for CausationId {
    fn from(value: CommandId) -> Self {
        Self(value.value())
    }
}
