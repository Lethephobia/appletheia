use serde::{Deserialize, Serialize};

/// Direction materialized by an account transaction fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountTransactionDirection {
    Incoming,
    Outgoing,
}
