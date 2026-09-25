use serde::{Deserialize, Serialize};

/// Status materialized by an account transaction fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountTransactionStatus {
    Pending,
    Completed,
    Failed,
    RequiresReview,
}
