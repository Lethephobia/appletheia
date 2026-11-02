use std::time::Duration as StdDuration;

use chrono::Duration;

use super::{ReadModelWatchRetryBackoffMultiplier, ReadModelWatchRetryDelayError};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReadModelWatchRetryDelay(Duration);

impl ReadModelWatchRetryDelay {
    pub fn default_max() -> Self {
        Self(Duration::seconds(5))
    }

    pub fn new(value: Duration) -> Result<Self, ReadModelWatchRetryDelayError> {
        if value <= Duration::zero() {
            return Err(ReadModelWatchRetryDelayError::NonPositive);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> Duration {
        self.0
    }

    pub fn to_std(self) -> StdDuration {
        StdDuration::new(self.0.num_seconds() as u64, self.0.subsec_nanos() as u32)
    }

    pub fn next(self, multiplier: ReadModelWatchRetryBackoffMultiplier, max: Self) -> Self {
        let seconds = self.to_std().as_secs_f64() * multiplier.value();
        if seconds >= max.to_std().as_secs_f64() {
            return max;
        }
        // The value is positive and bounded by a representable chrono duration.
        let seconds_part = seconds.floor() as i64;
        let nanos = ((seconds - seconds.floor()) * 1_000_000_000.0) as u32;
        Self(Duration::new(seconds_part, nanos).unwrap_or(max.0))
            .max(self)
            .min(max)
    }
}

impl TryFrom<Duration> for ReadModelWatchRetryDelay {
    type Error = ReadModelWatchRetryDelayError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Default for ReadModelWatchRetryDelay {
    fn default() -> Self {
        Self(Duration::milliseconds(100))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_positive_delays_and_caps_backoff() {
        assert!(ReadModelWatchRetryDelay::new(Duration::zero()).is_err());
        assert!(ReadModelWatchRetryDelay::new(Duration::nanoseconds(-1)).is_err());
        let multiplier = ReadModelWatchRetryBackoffMultiplier::default();
        let max = ReadModelWatchRetryDelay::new(Duration::milliseconds(250)).unwrap();
        let delay = ReadModelWatchRetryDelay::default();
        let next = delay.next(multiplier, max);
        assert_eq!(next.value(), Duration::milliseconds(200));
        assert_eq!(next.next(multiplier, max), max);
        assert_eq!(max.next(multiplier, max), max);
        assert_eq!(delay.next(f64::MAX.try_into().unwrap(), max), max);
        let tiny = ReadModelWatchRetryDelay::new(Duration::nanoseconds(1)).unwrap();
        assert_eq!(tiny.next(1.0.try_into().unwrap(), max), tiny);
        for value in [f64::NAN, f64::INFINITY, 0.5] {
            assert!(ReadModelWatchRetryBackoffMultiplier::try_from(value).is_err());
        }
    }
}
