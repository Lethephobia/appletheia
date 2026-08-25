use serde::{Deserialize, Serialize};

use super::EvmTransactionRequest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EthereumDepositSettlementPreparation {
    pub transaction_request: EvmTransactionRequest,
}
