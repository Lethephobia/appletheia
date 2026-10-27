use super::ReadModelWatchRetryBackoffMultiplierError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReadModelWatchRetryBackoffMultiplier(f64);

impl ReadModelWatchRetryBackoffMultiplier {
    pub fn new() -> Self {
        Self(2.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Default for ReadModelWatchRetryBackoffMultiplier {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for ReadModelWatchRetryBackoffMultiplier {
    type Error = ReadModelWatchRetryBackoffMultiplierError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value < 1.0 {
            return Err(ReadModelWatchRetryBackoffMultiplierError::Invalid(value));
        }
        Ok(ReadModelWatchRetryBackoffMultiplier(value))
    }
}
