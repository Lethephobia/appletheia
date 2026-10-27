use super::OutboxRetryBackoffMultiplierError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutboxRetryBackoffMultiplier(f64);

impl OutboxRetryBackoffMultiplier {
    pub fn new() -> Self {
        Self(2.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Default for OutboxRetryBackoffMultiplier {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for OutboxRetryBackoffMultiplier {
    type Error = OutboxRetryBackoffMultiplierError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value < 1.0 {
            return Err(OutboxRetryBackoffMultiplierError::Invalid(value));
        }
        Ok(OutboxRetryBackoffMultiplier(value))
    }
}
