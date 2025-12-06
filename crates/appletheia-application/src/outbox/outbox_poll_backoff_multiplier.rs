use super::OutboxPollBackoffMultiplierError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutboxPollBackoffMultiplier(f64);

impl OutboxPollBackoffMultiplier {
    pub fn new() -> Self {
        Self(1.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for OutboxPollBackoffMultiplier {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for OutboxPollBackoffMultiplier {
    type Error = OutboxPollBackoffMultiplierError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value < 1.0 {
            return Err(OutboxPollBackoffMultiplierError::Invalid(value));
        }
        Ok(OutboxPollBackoffMultiplier(value))
    }
}
