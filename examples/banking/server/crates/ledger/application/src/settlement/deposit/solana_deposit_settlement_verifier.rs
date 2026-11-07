use super::{
    DepositSettlementVerifierError, SolanaDepositSettlementVerification,
    SolanaDepositSettlementVerificationRequest,
};

#[allow(async_fn_in_trait)]
pub trait SolanaDepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: SolanaDepositSettlementVerificationRequest,
    ) -> Result<SolanaDepositSettlementVerification, DepositSettlementVerifierError>;
}
