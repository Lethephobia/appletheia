use serde::{Deserialize, Serialize};

/// Describes progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyIssuanceSagaStatus {
    #[default]
    Initial,
    Issued,
    SupplyReserved,
    MintSupplySynced,
    Deposited,
    SupplyCommitRequested,
    SupplyCommitted,
    SupplyReleaseRequested,
    SupplyReleased,
    Completed,
    Failed,
}
