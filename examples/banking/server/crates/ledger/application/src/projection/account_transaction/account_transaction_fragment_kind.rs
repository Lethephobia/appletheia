use serde::{Deserialize, Serialize};

/// Kind stored by an account transaction fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountTransactionFragmentKind {
    Deposit,
    Withdrawal,
    Transfer,
}
