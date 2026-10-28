use alloy::primitives::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::ethereum) struct EvmUserOperationGasEstimation {
    pub(in crate::ethereum) pre_verification_gas: U256,
    #[serde(alias = "verificationGasLimit")]
    pub(in crate::ethereum) verification_gas: U256,
    pub(in crate::ethereum) paymaster_verification_gas: U256,
    pub(in crate::ethereum) call_gas_limit: U256,
}
