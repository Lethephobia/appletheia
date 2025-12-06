use super::OutboxPollJitterRatioError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutboxPollJitterRatio(f64);

impl OutboxPollJitterRatio {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for OutboxPollJitterRatio {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<f64> for OutboxPollJitterRatio {
    type Error = OutboxPollJitterRatioError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(OutboxPollJitterRatioError::Invalid(value));
        }
        Ok(OutboxPollJitterRatio(value))
    }
}
