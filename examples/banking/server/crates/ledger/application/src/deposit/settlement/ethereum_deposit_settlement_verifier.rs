use super::{
    DepositSettlementVerifierError, EthereumDepositSettlementVerification,
    EthereumDepositSettlementVerifyRequest,
};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: EthereumDepositSettlementVerifyRequest,
    ) -> Result<EthereumDepositSettlementVerification, DepositSettlementVerifierError>;
}
