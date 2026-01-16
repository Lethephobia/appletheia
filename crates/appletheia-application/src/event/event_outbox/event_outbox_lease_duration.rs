use chrono::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventOutboxLeaseDuration(Duration);

impl EventOutboxLeaseDuration {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for EventOutboxLeaseDuration {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<EventOutboxLeaseDuration> for Duration {
    fn from(value: EventOutboxLeaseDuration) -> Self {
        value.value()
    }
}
