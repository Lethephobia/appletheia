use std::time::Duration as StdDuration;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::ReadYourWritesPollIntervalError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Duration", into = "Duration")]
pub struct ReadYourWritesPollInterval(Duration);

impl ReadYourWritesPollInterval {
    pub fn new(value: Duration) -> Result<Self, ReadYourWritesPollIntervalError> {
        if value < Duration::zero() {
            return Err(ReadYourWritesPollIntervalError::Negative);
        }

        value
            .to_std()
            .map_err(|_| ReadYourWritesPollIntervalError::OutOfRange)?;

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<StdDuration> for ReadYourWritesPollInterval {
    fn from(value: StdDuration) -> Self {
        let duration = Duration::from_std(value)
            .expect("std::time::Duration should fit within chrono::Duration");
        Self::new(duration).expect("std::time::Duration should be a valid poll interval")
    }
}

impl TryFrom<Duration> for ReadYourWritesPollInterval {
    type Error = ReadYourWritesPollIntervalError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadYourWritesPollInterval> for Duration {
    fn from(value: ReadYourWritesPollInterval) -> Self {
        value.value()
    }
}

impl From<ReadYourWritesPollInterval> for StdDuration {
    fn from(value: ReadYourWritesPollInterval) -> Self {
        value.value().to_std().expect(
            "validated read-your-writes poll interval should fit within std::time::Duration",
        )
    }
}
