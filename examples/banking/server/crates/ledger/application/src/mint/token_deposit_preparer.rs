use super::{TokenDepositPreparation, TokenDepositPrepareRequest, TokenDepositPreparerError};

#[allow(async_fn_in_trait)]
pub trait TokenDepositPreparer: Send + Sync {
    async fn prepare(
        &self,
        request: TokenDepositPrepareRequest,
    ) -> Result<TokenDepositPreparation, TokenDepositPreparerError>;
}
