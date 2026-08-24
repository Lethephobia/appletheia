use serde::{Deserialize, Serialize};

/// Describes why removing a CurrencyRegistrarMembership was rejected.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyRegistrarMembershipRemoveRejectionReason {
    AlreadyRemoved,
}
