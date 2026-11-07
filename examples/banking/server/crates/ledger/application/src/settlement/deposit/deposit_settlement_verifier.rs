use super::{
    DepositSettlementVerification, DepositSettlementVerificationRequest,
    DepositSettlementVerifierError,
};

#[allow(async_fn_in_trait)]
pub trait DepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: DepositSettlementVerificationRequest,
    ) -> Result<DepositSettlementVerification, DepositSettlementVerifierError>;
}
