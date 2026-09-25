use super::{
    DepositSettlementVerifierError, SolanaDepositSettlementVerification,
    SolanaDepositSettlementVerifyRequest,
};

#[allow(async_fn_in_trait)]
pub trait SolanaDepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: SolanaDepositSettlementVerifyRequest,
    ) -> Result<SolanaDepositSettlementVerification, DepositSettlementVerifierError>;
}
