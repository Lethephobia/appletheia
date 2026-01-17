use super::{
    OutboxPollBackoffMultiplier, OutboxPollInterval, OutboxPollJitterRatio,
    OutboxPollingOptionsError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct OutboxPollingOptions {
    pub base: OutboxPollInterval,
    pub max: OutboxPollInterval,
    pub multiplier: OutboxPollBackoffMultiplier,
    pub jitter: OutboxPollJitterRatio,
}

impl OutboxPollingOptions {
    pub fn new(
        base: OutboxPollInterval,
        max: OutboxPollInterval,
        multiplier: OutboxPollBackoffMultiplier,
        jitter: OutboxPollJitterRatio,
    ) -> Result<Self, OutboxPollingOptionsError> {
        if base.value() > max.value() {
            return Err(OutboxPollingOptionsError::BaseGreaterThanMax { base, max });
        }

        Ok(Self {
            base,
            max,
            multiplier,
            jitter,
        })
    }
}
