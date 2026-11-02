use banking_ledger_domain::core::{EvmAddress, EvmTokenOwnerAddress};
use serde::{Deserialize, Serialize};

use super::{EvmCallData, EvmQuantity};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmUserOperation {
    pub sender: EvmTokenOwnerAddress,
    pub nonce: EvmQuantity,
    pub call_data: EvmCallData,
    pub call_gas_limit: EvmQuantity,
    pub verification_gas_limit: EvmQuantity,
    pub pre_verification_gas: EvmQuantity,
    pub max_fee_per_gas: EvmQuantity,
    pub max_priority_fee_per_gas: EvmQuantity,
    pub paymaster: EvmAddress,
    pub paymaster_verification_gas_limit: EvmQuantity,
    pub paymaster_post_op_gas_limit: EvmQuantity,
    pub paymaster_data: EvmCallData,
}
