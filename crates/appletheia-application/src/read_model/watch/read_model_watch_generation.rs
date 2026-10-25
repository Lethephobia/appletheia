use std::{fmt, fmt::Display};

use super::ReadModelWatchGenerationError;

/// A non-negative, monotonically increasing generation within one watch.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReadModelWatchGeneration(i64);

impl ReadModelWatchGeneration {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn try_next(self) -> Result<Self, ReadModelWatchGenerationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(ReadModelWatchGenerationError::Overflow)
    }
}

impl Default for ReadModelWatchGeneration {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<i64> for ReadModelWatchGeneration {
    type Error = ReadModelWatchGenerationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(ReadModelWatchGenerationError::NegativeValue(value));
        }
        Ok(Self(value))
    }
}

impl From<ReadModelWatchGeneration> for i64 {
    fn from(value: ReadModelWatchGeneration) -> Self {
        value.value()
    }
}

impl Display for ReadModelWatchGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}
