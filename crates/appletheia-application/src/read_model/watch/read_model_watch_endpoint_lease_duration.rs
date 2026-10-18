use chrono::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadModelWatchEndpointLeaseDuration(Duration);

impl ReadModelWatchEndpointLeaseDuration {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for ReadModelWatchEndpointLeaseDuration {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<ReadModelWatchEndpointLeaseDuration> for Duration {
    fn from(value: ReadModelWatchEndpointLeaseDuration) -> Self {
        value.value()
    }
}
