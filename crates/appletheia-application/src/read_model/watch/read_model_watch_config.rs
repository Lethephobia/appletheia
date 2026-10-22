use super::{
    ReadModelWatchConfigError, ReadModelWatchLeaseDuration, ReadModelWatchLeaseRefreshInterval,
};

#[derive(Clone, Debug, Default)]
pub struct ReadModelWatchConfig {
    lease_duration: ReadModelWatchLeaseDuration,
    lease_refresh_interval: ReadModelWatchLeaseRefreshInterval,
}

impl ReadModelWatchConfig {
    pub fn new(
        lease_duration: ReadModelWatchLeaseDuration,
        lease_refresh_interval: ReadModelWatchLeaseRefreshInterval,
    ) -> Result<Self, ReadModelWatchConfigError> {
        if lease_refresh_interval.value() >= lease_duration.value() {
            return Err(ReadModelWatchConfigError::RefreshIntervalTooLong);
        }
        Ok(Self {
            lease_duration,
            lease_refresh_interval,
        })
    }

    pub fn lease_duration(&self) -> ReadModelWatchLeaseDuration {
        self.lease_duration
    }

    pub fn lease_refresh_interval(&self) -> ReadModelWatchLeaseRefreshInterval {
        self.lease_refresh_interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn refresh_interval_must_be_shorter_than_lease() {
        for seconds in [60, 61] {
            assert!(
                ReadModelWatchConfig::new(
                    Default::default(),
                    Duration::seconds(seconds).try_into().unwrap(),
                )
                .is_err()
            );
        }
        assert!(ReadModelWatchConfig::new(Default::default(), Default::default()).is_ok());
    }
}
