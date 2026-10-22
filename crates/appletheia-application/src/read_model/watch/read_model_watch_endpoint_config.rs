use std::num::NonZeroU32;

use super::{
    ReadModelWatchEndpointConfigError, ReadModelWatchEndpointLeaseDuration,
    ReadModelWatchEndpointLeaseRefreshInterval, ReadModelWatchPageSize,
};

#[derive(Clone, Debug)]
pub struct ReadModelWatchEndpointConfig {
    endpoint_lease_duration: ReadModelWatchEndpointLeaseDuration,
    endpoint_lease_refresh_interval: ReadModelWatchEndpointLeaseRefreshInterval,
    watch_page_size: ReadModelWatchPageSize,
}

impl ReadModelWatchEndpointConfig {
    pub fn new(
        endpoint_lease_duration: ReadModelWatchEndpointLeaseDuration,
        endpoint_lease_refresh_interval: ReadModelWatchEndpointLeaseRefreshInterval,
        watch_page_size: ReadModelWatchPageSize,
    ) -> Result<Self, ReadModelWatchEndpointConfigError> {
        if endpoint_lease_refresh_interval.value() >= endpoint_lease_duration.value() {
            return Err(ReadModelWatchEndpointConfigError::EndpointRefreshIntervalTooLong);
        }
        Ok(Self {
            endpoint_lease_duration,
            endpoint_lease_refresh_interval,
            watch_page_size,
        })
    }

    pub fn endpoint_lease_duration(&self) -> ReadModelWatchEndpointLeaseDuration {
        self.endpoint_lease_duration
    }

    pub fn endpoint_lease_refresh_interval(&self) -> ReadModelWatchEndpointLeaseRefreshInterval {
        self.endpoint_lease_refresh_interval
    }

    pub fn watch_page_size(&self) -> ReadModelWatchPageSize {
        self.watch_page_size
    }
}

impl Default for ReadModelWatchEndpointConfig {
    fn default() -> Self {
        Self {
            endpoint_lease_duration: Default::default(),
            endpoint_lease_refresh_interval: Default::default(),
            watch_page_size: ReadModelWatchPageSize::new(NonZeroU32::MIN.saturating_add(99)),
        }
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
                ReadModelWatchEndpointConfig::new(
                    Default::default(),
                    Duration::seconds(seconds).try_into().unwrap(),
                    ReadModelWatchEndpointConfig::default().watch_page_size(),
                )
                .is_err()
            );
        }
        assert!(
            ReadModelWatchEndpointConfig::new(
                Default::default(),
                Default::default(),
                ReadModelWatchEndpointConfig::default().watch_page_size(),
            )
            .is_ok()
        );
    }
}
