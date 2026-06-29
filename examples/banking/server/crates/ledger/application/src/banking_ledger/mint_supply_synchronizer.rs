use super::{MintSupplySyncRequest, MintSupplySynchronizerError};

/// Synchronizes on-chain mint supply into the internal pool account.
#[allow(async_fn_in_trait)]
pub trait MintSupplySynchronizer: Send + Sync {
    async fn sync_supply(
        &self,
        request: MintSupplySyncRequest,
    ) -> Result<(), MintSupplySynchronizerError>;
}
