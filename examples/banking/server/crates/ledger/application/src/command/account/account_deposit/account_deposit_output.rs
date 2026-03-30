use serde::{Deserialize, Serialize};

/// Returned after an account deposit request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountDepositOutput;
