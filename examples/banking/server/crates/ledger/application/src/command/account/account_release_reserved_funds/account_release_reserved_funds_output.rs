use serde::{Deserialize, Serialize};

/// Returned after releasing reserved funds in an account.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReleaseReservedFundsOutput;
