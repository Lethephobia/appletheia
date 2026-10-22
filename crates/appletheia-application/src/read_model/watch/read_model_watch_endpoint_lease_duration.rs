use chrono::Duration;

use super::ReadModelWatchEndpointLeaseDurationError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadModelWatchEndpointLeaseDuration(Duration);

impl ReadModelWatchEndpointLeaseDuration {
    pub fn new(value: Duration) -> Result<Self, ReadModelWatchEndpointLeaseDurationError> {
        if value <= Duration::zero() {
            return Err(ReadModelWatchEndpointLeaseDurationError::NonPositive);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl Default for ReadModelWatchEndpointLeaseDuration {
    fn default() -> Self {
        Self(Duration::seconds(60))
    }
}

impl TryFrom<Duration> for ReadModelWatchEndpointLeaseDuration {
    type Error = ReadModelWatchEndpointLeaseDurationError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelWatchEndpointLeaseDuration> for Duration {
    fn from(value: ReadModelWatchEndpointLeaseDuration) -> Self {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_positive_values_and_accepts_positive_values() {
        assert!(matches!(
            ReadModelWatchEndpointLeaseDuration::new(Duration::zero()),
            Err(ReadModelWatchEndpointLeaseDurationError::NonPositive)
        ));
        assert!(ReadModelWatchEndpointLeaseDuration::try_from(Duration::nanoseconds(-1)).is_err());
        assert_eq!(
            ReadModelWatchEndpointLeaseDuration::try_from(Duration::nanoseconds(1))
                .unwrap()
                .value(),
            Duration::nanoseconds(1)
        );
    }
}
