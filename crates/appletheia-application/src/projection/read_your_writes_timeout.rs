use std::time::Duration as StdDuration;

use chrono::Duration;

use super::ReadYourWritesTimeoutError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadYourWritesTimeout(StdDuration);

impl ReadYourWritesTimeout {
    pub fn new(value: StdDuration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> StdDuration {
        self.0
    }
}

impl From<StdDuration> for ReadYourWritesTimeout {
    fn from(value: StdDuration) -> Self {
        Self::new(value)
    }
}

impl From<ReadYourWritesTimeout> for StdDuration {
    fn from(value: ReadYourWritesTimeout) -> Self {
        value.value()
    }
}

impl TryFrom<Duration> for ReadYourWritesTimeout {
    type Error = ReadYourWritesTimeoutError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        if value < Duration::zero() {
            return Err(ReadYourWritesTimeoutError::Negative);
        }

        let std = value
            .to_std()
            .map_err(|_| ReadYourWritesTimeoutError::OutOfRange)?;

        Ok(Self::new(std))
    }
}
