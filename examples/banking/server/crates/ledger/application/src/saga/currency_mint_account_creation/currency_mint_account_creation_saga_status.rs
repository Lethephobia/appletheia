use serde::{Deserialize, Serialize};

/// Describes progress for the currency mint account creation saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyMintAccountCreationSagaStatus {
    #[default]
    Initial,
    Defined,
    Completed,
    Failed,
}
