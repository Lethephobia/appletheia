use chrono::Duration;

use super::ReadModelWatchLeaseDurationError;

/// Sets how long index registrations remain valid without renewal.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_and_negative_durations() {
        for value in [Duration::zero(), Duration::seconds(-1)] {
            assert!(matches!(
                ReadModelWatchLeaseDuration::new(value),
                Err(ReadModelWatchLeaseDurationError::NonPositive),
            ));
        }
    }
}
