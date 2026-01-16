use chrono::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventOutboxRetryDelay(Duration);

impl EventOutboxRetryDelay {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for EventOutboxRetryDelay {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<EventOutboxRetryDelay> for Duration {
    fn from(value: EventOutboxRetryDelay) -> Self {
        value.value()
    }
}
