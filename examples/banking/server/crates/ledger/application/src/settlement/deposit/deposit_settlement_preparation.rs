use serde::{Deserialize, Serialize};

use super::PreparedDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DepositSettlementPreparation {
    pub transaction: PreparedDepositTransaction,
}
