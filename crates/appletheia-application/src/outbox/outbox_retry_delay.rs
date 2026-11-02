use chrono::Duration;

use super::{OutboxAttemptCount, OutboxRetryBackoffMultiplier};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OutboxRetryDelay(Duration);

impl OutboxRetryDelay {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(self) -> Duration {
        self.0
    }
    pub fn for_attempt(
        self,
        attempt_count: OutboxAttemptCount,
        multiplier: OutboxRetryBackoffMultiplier,
        max: Self,
    ) -> Self {
        let base = self
            .0
            .max(Duration::zero())
            .min(max.0.max(Duration::zero()));
        if base == Duration::zero() {
            return Self(base);
        }
        let exponent = attempt_count.value().saturating_sub(1).max(0) as f64;
        let seconds = base.as_seconds_f64() * multiplier.value().powf(exponent);
        if seconds >= max.0.as_seconds_f64() {
            return max;
        }
        let seconds_part = seconds.floor() as i64;
        let nanos = ((seconds - seconds.floor()) * 1_000_000_000.0) as u32;
        Self(
            Duration::new(seconds_part, nanos)
                .unwrap_or(max.0)
                .max(base)
                .min(max.0),
        )
    }
}

impl From<Duration> for OutboxRetryDelay {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<OutboxRetryDelay> for Duration {
    fn from(value: OutboxRetryDelay) -> Self {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grows_from_first_failure_and_caps_at_maximum() {
        let base = OutboxRetryDelay::new(Duration::milliseconds(100));
        let max = OutboxRetryDelay::new(Duration::milliseconds(500));
        let multiplier = OutboxRetryBackoffMultiplier::default();
        for (attempt, milliseconds) in [(1, 100), (2, 200), (3, 400), (4, 500), (100, 500)] {
            assert_eq!(
                base.for_attempt(attempt.try_into().unwrap(), multiplier, max)
                    .value(),
                Duration::milliseconds(milliseconds)
            );
        }
        let restored_attempt = OutboxAttemptCount::try_from(3).unwrap();
        assert_eq!(
            base.for_attempt(restored_attempt, multiplier, max).value(),
            Duration::milliseconds(400)
        );
    }

    #[test]
    fn handles_constant_zero_and_extreme_backoff() {
        let base = OutboxRetryDelay::new(Duration::seconds(1));
        let max = OutboxRetryDelay::new(Duration::seconds(5));
        let attempt = OutboxAttemptCount::try_from(i64::MAX).unwrap();
        assert_eq!(
            base.for_attempt(attempt, 1.0.try_into().unwrap(), max),
            base
        );
        assert_eq!(
            base.for_attempt(attempt, f64::MAX.try_into().unwrap(), max),
            max
        );
        assert_eq!(
            max.for_attempt(1.try_into().unwrap(), Default::default(), base),
            base
        );
        let zero = OutboxRetryDelay::new(Duration::zero());
        assert_eq!(
            zero.for_attempt(attempt, f64::MAX.try_into().unwrap(), max),
            zero
        );
        for invalid in [f64::NAN, f64::INFINITY, 0.5] {
            assert!(OutboxRetryBackoffMultiplier::try_from(invalid).is_err());
        }
    }
}
