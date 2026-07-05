use serde::{Deserialize, Serialize};

/// Describes why a mint metadata sync was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum MintMetadataSyncRejectionReason {
    NotProvisioned,
}
