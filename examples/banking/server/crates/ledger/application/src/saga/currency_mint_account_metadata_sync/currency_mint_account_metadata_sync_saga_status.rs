use serde::{Deserialize, Serialize};

/// Describes progress for the currency mint account metadata sync saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyMintAccountMetadataSyncSagaStatus {
    SyncRequested,
    Synced,
    NotProvisioned,
}
