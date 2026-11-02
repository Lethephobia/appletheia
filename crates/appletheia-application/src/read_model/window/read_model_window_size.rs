use serde::{Deserialize, Serialize};

use super::ReadModelWindowSizeError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct ReadModelWindowSize(u32);

impl ReadModelWindowSize {
    pub fn new(value: u32) -> Result<Self, ReadModelWindowSizeError> {
        Self::try_from(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for ReadModelWindowSize {
    type Error = ReadModelWindowSizeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(ReadModelWindowSizeError::Zero);
        }
        Ok(Self(value))
    }
}

impl From<ReadModelWindowSize> for u32 {
    fn from(value: ReadModelWindowSize) -> Self {
        value.0
    }
}
