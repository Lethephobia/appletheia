use super::SolanaPreparedDepositTransaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SolanaDepositSettlementPreparation {
    pub transaction: SolanaPreparedDepositTransaction,
}
