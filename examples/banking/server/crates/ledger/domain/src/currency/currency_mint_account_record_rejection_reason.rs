use serde::{Deserialize, Serialize};

/// Describes why recording a currency mint account was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyMintAccountRecordRejectionReason {
    AlreadyRecorded,
    Removed,
}
