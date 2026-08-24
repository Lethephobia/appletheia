use banking_ledger_domain::core::{EthereumNetwork, EvmTokenContractAddress};

use super::EthereumTokenContractInspection;

#[allow(async_fn_in_trait)]
pub trait EthereumTokenContractInspector: Send + Sync {
    async fn inspect(
        &self,
        network: EthereumNetwork,
        token: &EvmTokenContractAddress,
    ) -> Result<EthereumTokenContractInspection, Box<dyn std::error::Error + Send + Sync>>;
}
