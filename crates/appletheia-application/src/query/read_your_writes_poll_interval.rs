use std::time::Duration as StdDuration;

use chrono::Duration;

use super::ReadYourWritesPollIntervalError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadYourWritesPollInterval(StdDuration);

impl ReadYourWritesPollInterval {
    pub fn new(value: StdDuration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> StdDuration {
        self.0
    }
}

impl From<StdDuration> for ReadYourWritesPollInterval {
    fn from(value: StdDuration) -> Self {
        Self::new(value)
    }
}

impl From<ReadYourWritesPollInterval> for StdDuration {
    fn from(value: ReadYourWritesPollInterval) -> Self {
        value.value()
    }
}

impl TryFrom<Duration> for ReadYourWritesPollInterval {
    type Error = ReadYourWritesPollIntervalError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        if value < Duration::zero() {
            return Err(ReadYourWritesPollIntervalError::Negative);
        }

        let std = value
            .to_std()
            .map_err(|_| ReadYourWritesPollIntervalError::OutOfRange)?;

        Ok(Self::new(std))
    }
}

