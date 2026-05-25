use serde::{Deserialize, Serialize};

/// Describes progress for the currency provisioning saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyProvisioningSagaStatus {
    #[default]
    Initial,
    Defined,
    Completed,
    Failed,
}
