use chrono::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutboxLeaseDuration(Duration);

impl OutboxLeaseDuration {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for OutboxLeaseDuration {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<OutboxLeaseDuration> for Duration {
    fn from(value: OutboxLeaseDuration) -> Self {
        value.value()
    }
}
