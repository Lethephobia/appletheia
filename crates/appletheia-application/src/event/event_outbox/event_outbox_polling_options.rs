use super::{
    EventOutboxPollBackoffMultiplier, EventOutboxPollInterval, EventOutboxPollJitterRatio,
    EventOutboxPollingOptionsError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct EventOutboxPollingOptions {
    pub base: EventOutboxPollInterval,
    pub max: EventOutboxPollInterval,
    pub multiplier: EventOutboxPollBackoffMultiplier,
    pub jitter: EventOutboxPollJitterRatio,
}

impl EventOutboxPollingOptions {
    pub fn new(
        base: EventOutboxPollInterval,
        max: EventOutboxPollInterval,
        multiplier: EventOutboxPollBackoffMultiplier,
        jitter: EventOutboxPollJitterRatio,
    ) -> Result<Self, EventOutboxPollingOptionsError> {
        if base.value() > max.value() {
            return Err(EventOutboxPollingOptionsError::BaseGreaterThanMax { base, max });
        }

        Ok(Self {
            base,
            max,
            multiplier,
            jitter,
        })
    }
}
