use serde::{Deserialize, Serialize};

/// Describes progress for the currency issuance orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyIssuanceSagaStatus {
    #[default]
    Initial,
    SupplyReserveRequested,
    MintSupplySyncRequested,
    DepositRequested,
    SupplyCommitRequested,
    SupplyReleaseRequested,
    SupplyReleaseMintSupplySyncRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
