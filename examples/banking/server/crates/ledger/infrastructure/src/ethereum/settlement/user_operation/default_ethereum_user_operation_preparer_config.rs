use alloy::signers::local::PrivateKeySigner;
use banking_ledger_domain::core::EvmAddress;
use chrono::Duration;

pub struct DefaultEthereumUserOperationPreparerConfig {
    pub(super) paymaster_contract: EvmAddress,
    pub(super) sponsorship_signer: PrivateKeySigner,
    pub(super) sponsorship_signature_ttl: Duration,
    pub(super) paymaster_verification_gas_limit: u128,
    pub(super) paymaster_post_op_gas_limit: u128,
}

impl DefaultEthereumUserOperationPreparerConfig {
    pub fn new(
        paymaster_contract: EvmAddress,
        sponsorship_signer: PrivateKeySigner,
        sponsorship_signature_ttl: Duration,
        paymaster_verification_gas_limit: u128,
        paymaster_post_op_gas_limit: u128,
    ) -> Self {
        Self {
            paymaster_contract,
            sponsorship_signer,
            sponsorship_signature_ttl,
            paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit,
        }
    }
}
