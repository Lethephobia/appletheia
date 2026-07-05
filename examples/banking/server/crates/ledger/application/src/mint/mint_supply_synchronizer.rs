use super::{MintSupplySyncRequest, MintSupplySynchronizerError};

#[allow(async_fn_in_trait)]
pub trait MintSupplySynchronizer: Send + Sync {
    async fn sync_supply(
        &self,
        request: MintSupplySyncRequest,
    ) -> Result<(), MintSupplySynchronizerError>;
}
