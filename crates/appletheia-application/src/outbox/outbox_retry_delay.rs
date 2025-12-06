use chrono::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OutboxRetryDelay(Duration);

impl OutboxRetryDelay {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for OutboxRetryDelay {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<OutboxRetryDelay> for Duration {
    fn from(value: OutboxRetryDelay) -> Self {
        value.value()
    }
}
