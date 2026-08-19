use std::time::Duration;

use super::ReadModelWatchLimitsError;

/// Bounds process-local state retained for read-model watch delivery.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchLimits {
    pub max_subscriptions_per_session: usize,
    pub max_active_chunks_per_list_subscription: usize,
    pub max_materialized_dependencies_per_subscription: usize,
    pub max_concurrent_refreshes: usize,
    pub max_concurrent_deliveries: usize,
    pub session_idle_ttl: Duration,
}

impl Default for ReadModelWatchLimits {
    fn default() -> Self {
        Self {
            max_subscriptions_per_session: 128,
            max_active_chunks_per_list_subscription: 7,
            max_materialized_dependencies_per_subscription: 4_096,
            max_concurrent_refreshes: 32,
            max_concurrent_deliveries: 128,
            session_idle_ttl: Duration::from_secs(90),
        }
    }
}

impl ReadModelWatchLimits {
    pub(super) fn validate(&self) -> Result<(), ReadModelWatchLimitsError> {
        for (name, value) in [
            (
                "max_subscriptions_per_session",
                self.max_subscriptions_per_session,
            ),
            (
                "max_active_chunks_per_list_subscription",
                self.max_active_chunks_per_list_subscription,
            ),
            (
                "max_materialized_dependencies_per_subscription",
                self.max_materialized_dependencies_per_subscription,
            ),
            ("max_concurrent_refreshes", self.max_concurrent_refreshes),
            ("max_concurrent_deliveries", self.max_concurrent_deliveries),
        ] {
            if value == 0 {
                return Err(ReadModelWatchLimitsError::Zero(name));
            }
        }
        if self.session_idle_ttl.is_zero() {
            return Err(ReadModelWatchLimitsError::Zero("session_idle_ttl"));
        }
        Ok(())
    }
}
