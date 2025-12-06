use chrono::Duration;

use super::{OutboxPollBackoffMultiplier, OutboxPollJitterRatio};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutboxPollInterval(Duration);

impl OutboxPollInterval {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }

    pub fn next(
        self,
        multiplier: OutboxPollBackoffMultiplier,
        jitter: OutboxPollJitterRatio,
        max: OutboxPollInterval,
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

        OutboxPollInterval::new(next_duration)
    }
}

impl From<Duration> for OutboxPollInterval {
    fn from(value: Duration) -> Self {
        OutboxPollInterval::new(value)
    }
}

impl From<OutboxPollInterval> for Duration {
    fn from(value: OutboxPollInterval) -> Self {
        value.value()
    }
}
