use serde::{Deserialize, Serialize};

use super::TokenBindingRemoveRejectionReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TokenBindingRemoveResult {
    Removed,
    Rejected {
        reason: TokenBindingRemoveRejectionReason,
    },
}
