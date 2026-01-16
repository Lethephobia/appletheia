use chrono::Duration;

use super::{EventOutboxPollBackoffMultiplier, EventOutboxPollJitterRatio};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EventOutboxPollInterval(Duration);

impl EventOutboxPollInterval {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }

    pub fn next(
        self,
        multiplier: EventOutboxPollBackoffMultiplier,
        jitter: EventOutboxPollJitterRatio,
        max: EventOutboxPollInterval,
    ) -> Self {
        let current = self.value();
        let mut next_ms = current.num_milliseconds() as f64;
        let max_ms = max.value().num_milliseconds() as f64;

        let m = multiplier.value();
        next_ms = (next_ms * m).max(0.0);

        let jitter_ratio = jitter.value();
        if jitter_ratio > 0.0 {
            let jitter_factor = 1.0 + jitter_ratio;
            next_ms *= jitter_factor;
        }

        if next_ms > max_ms {
            next_ms = max_ms;
        }

        let clamped_ms = next_ms.round() as i64;
        let next_duration = Duration::milliseconds(clamped_ms.max(0));

        EventOutboxPollInterval::new(next_duration)
    }
}

impl From<Duration> for EventOutboxPollInterval {
    fn from(value: Duration) -> Self {
        EventOutboxPollInterval::new(value)
    }
}

impl From<EventOutboxPollInterval> for Duration {
    fn from(value: EventOutboxPollInterval) -> Self {
        value.value()
    }
}
