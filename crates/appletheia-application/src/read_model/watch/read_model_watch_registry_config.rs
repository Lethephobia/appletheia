use super::ReadModelWatchLeaseDuration;

/// Configures the registry's registration lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchRegistryConfig {
    pub lease_duration: ReadModelWatchLeaseDuration,
}
