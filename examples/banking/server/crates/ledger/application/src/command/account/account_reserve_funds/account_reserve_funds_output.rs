use serde::{Deserialize, Serialize};

/// Returned after reserving funds in an account.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReserveFundsOutput;
