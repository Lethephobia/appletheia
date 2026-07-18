use serde::{Deserialize, Serialize};

/// Describes why a currency-image upload preparation was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyImageUploadPrepareRejectionReason {
    CurrencyRemoved,
    ContentLengthTooLarge,
    ContentTypeNotAllowed,
}
