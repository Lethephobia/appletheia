use chrono::Duration;

use super::ReadModelWatchLeaseDurationError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadModelWatchLeaseDuration(Duration);

impl ReadModelWatchLeaseDuration {
    pub fn new(value: Duration) -> Result<Self, ReadModelWatchLeaseDurationError> {
        if value <= Duration::zero() {
            return Err(ReadModelWatchLeaseDurationError::NonPositive);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl Default for ReadModelWatchLeaseDuration {
    fn default() -> Self {
        Self(Duration::seconds(60))
    }
}

impl TryFrom<Duration> for ReadModelWatchLeaseDuration {
    type Error = ReadModelWatchLeaseDurationError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelWatchLeaseDuration> for Duration {
    fn from(value: ReadModelWatchLeaseDuration) -> Self {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_positive_values_and_accepts_positive_values() {
        assert!(matches!(
            ReadModelWatchLeaseDuration::new(Duration::zero()),
            Err(ReadModelWatchLeaseDurationError::NonPositive)
        ));
        assert!(ReadModelWatchLeaseDuration::try_from(Duration::nanoseconds(-1)).is_err());
        assert_eq!(
            ReadModelWatchLeaseDuration::try_from(Duration::nanoseconds(1))
                .unwrap()
                .value(),
            Duration::nanoseconds(1)
        );
    }
}
