use std::time::Duration as StdDuration;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::ReadYourWritesTimeoutError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Duration", into = "Duration")]
pub struct ReadYourWritesTimeout(Duration);

impl ReadYourWritesTimeout {
    pub fn new(value: Duration) -> Result<Self, ReadYourWritesTimeoutError> {
        if value < Duration::zero() {
            return Err(ReadYourWritesTimeoutError::Negative);
        }

        value
            .to_std()
            .map_err(|_| ReadYourWritesTimeoutError::OutOfRange)?;

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<StdDuration> for ReadYourWritesTimeout {
    fn from(value: StdDuration) -> Self {
        let duration = Duration::from_std(value)
            .expect("std::time::Duration should fit within chrono::Duration");
        Self::new(duration).expect("std::time::Duration should be a valid timeout")
    }
}

impl TryFrom<Duration> for ReadYourWritesTimeout {
    type Error = ReadYourWritesTimeoutError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadYourWritesTimeout> for Duration {
    fn from(value: ReadYourWritesTimeout) -> Self {
        value.value()
    }
}

impl From<ReadYourWritesTimeout> for StdDuration {
    fn from(value: ReadYourWritesTimeout) -> Self {
        value
            .value()
            .to_std()
            .expect("validated read-your-writes timeout should fit within std::time::Duration")
    }
}
