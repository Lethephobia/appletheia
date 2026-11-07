use super::{
    DepositSettlementVerifierError, EthereumDepositSettlementVerification,
    EthereumDepositSettlementVerificationRequest,
};

#[allow(async_fn_in_trait)]
pub trait EthereumDepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: EthereumDepositSettlementVerificationRequest,
    ) -> Result<EthereumDepositSettlementVerification, DepositSettlementVerifierError>;
}
