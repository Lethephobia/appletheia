use chrono::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadModelWatchLeaseDuration(Duration);

impl ReadModelWatchLeaseDuration {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for ReadModelWatchLeaseDuration {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<ReadModelWatchLeaseDuration> for Duration {
    fn from(value: ReadModelWatchLeaseDuration) -> Self {
        value.value()
    }
}
