use std::time::Duration as StdDuration;

use chrono::Duration;

use super::ReadModelWatchLeaseRefreshIntervalError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadModelWatchLeaseRefreshInterval(Duration);

impl ReadModelWatchLeaseRefreshInterval {
    pub fn new(value: Duration) -> Result<Self, ReadModelWatchLeaseRefreshIntervalError> {
        if value <= Duration::zero() {
            return Err(ReadModelWatchLeaseRefreshIntervalError::NonPositive);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }

    pub fn to_std(&self) -> StdDuration {
        StdDuration::new(self.0.num_seconds() as u64, self.0.subsec_nanos() as u32)
    }
}

impl Default for ReadModelWatchLeaseRefreshInterval {
    fn default() -> Self {
        Self(Duration::seconds(20))
    }
}

impl TryFrom<Duration> for ReadModelWatchLeaseRefreshInterval {
    type Error = ReadModelWatchLeaseRefreshIntervalError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelWatchLeaseRefreshInterval> for Duration {
    fn from(value: ReadModelWatchLeaseRefreshInterval) -> Self {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_positive_values_and_accepts_positive_values() {
        assert!(matches!(
            ReadModelWatchLeaseRefreshInterval::new(Duration::zero()),
            Err(ReadModelWatchLeaseRefreshIntervalError::NonPositive)
        ));
        assert!(ReadModelWatchLeaseRefreshInterval::try_from(Duration::nanoseconds(-1)).is_err());
        assert_eq!(
            ReadModelWatchLeaseRefreshInterval::try_from(Duration::nanoseconds(1))
                .unwrap()
                .value(),
            Duration::nanoseconds(1)
        );
    }
    #[test]
    fn std_conversion_preserves_subseconds_and_large_durations() {
        for duration in [
            Duration::nanoseconds(1),
            Duration::milliseconds(1501),
            Duration::MAX,
        ] {
            let interval = ReadModelWatchLeaseRefreshInterval::new(duration).unwrap();
            assert_eq!(interval.to_std(), duration.to_std().unwrap());
        }
    }
}
