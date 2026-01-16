use super::EventOutboxPollBackoffMultiplierError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EventOutboxPollBackoffMultiplier(f64);

impl EventOutboxPollBackoffMultiplier {
    pub fn new() -> Self {
        Self(1.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for EventOutboxPollBackoffMultiplier {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for EventOutboxPollBackoffMultiplier {
    type Error = EventOutboxPollBackoffMultiplierError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value < 1.0 {
            return Err(EventOutboxPollBackoffMultiplierError::Invalid(value));
        }
        Ok(EventOutboxPollBackoffMultiplier(value))
    }
}
