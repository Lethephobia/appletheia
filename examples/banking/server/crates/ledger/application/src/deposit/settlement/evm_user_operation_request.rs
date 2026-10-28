use banking_ledger_domain::core::{EvmAddress, EvmChainId};
use serde::{Deserialize, Serialize};

use super::EvmUserOperation;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvmUserOperationRequest {
    pub chain_id: EvmChainId,
    pub entry_point: EvmAddress,
    pub user_operation: EvmUserOperation,
}

impl EvmUserOperationRequest {
    pub const fn new(
        chain_id: EvmChainId,
        entry_point: EvmAddress,
        user_operation: EvmUserOperation,
    ) -> Self {
        Self {
            chain_id,
            entry_point,
            user_operation,
        }
    }
}
