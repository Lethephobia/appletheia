use super::EventOutboxPollJitterRatioError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EventOutboxPollJitterRatio(f64);

impl EventOutboxPollJitterRatio {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for EventOutboxPollJitterRatio {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for EventOutboxPollJitterRatio {
    type Error = EventOutboxPollJitterRatioError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(EventOutboxPollJitterRatioError::Invalid(value));
        }
        Ok(EventOutboxPollJitterRatio(value))
    }
}
