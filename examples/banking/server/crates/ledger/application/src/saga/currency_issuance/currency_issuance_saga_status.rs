use serde::{Deserialize, Serialize};

/// Describes progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyIssuanceSagaStatus {
    #[default]
    Initial,
    Issued,
    SupplyReserveRequested,
    SupplyReserved,
    MintSupplySyncRequested,
    MintSupplySynced,
    DepositRequested,
    Deposited,
    SupplyCommitRequested,
    SupplyCommitted,
    SupplyReleaseRequested,
    SupplyReleased,
    SupplyReleaseMintSupplySyncRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
