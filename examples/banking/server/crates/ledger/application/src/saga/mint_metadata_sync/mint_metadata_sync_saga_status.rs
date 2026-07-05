use serde::{Deserialize, Serialize};

/// Describes progress for the currency mint metadata sync saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintMetadataSyncSagaStatus {
    SyncRequested,
    Synced,
    NotProvisioned,
}
