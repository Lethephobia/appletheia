use super::{
    DepositSettlementVerification, DepositSettlementVerifierError, DepositSettlementVerifyRequest,
};

#[allow(async_fn_in_trait)]
pub trait DepositSettlementVerifier: Send + Sync {
    async fn verify(
        &self,
        request: DepositSettlementVerifyRequest,
    ) -> Result<DepositSettlementVerification, DepositSettlementVerifierError>;
}
