use serde::{Deserialize, Serialize};

/// Describes why requesting currency mint account creation was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyMintAccountCreationRequestRejectionReason {
    AlreadyRequested,
    AlreadyRecorded,
    Removed,
}
