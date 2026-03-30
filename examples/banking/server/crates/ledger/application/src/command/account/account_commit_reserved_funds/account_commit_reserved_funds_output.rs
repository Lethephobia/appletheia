use serde::{Deserialize, Serialize};

/// Returned after committing reserved funds in an account.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountCommitReservedFundsOutput;
