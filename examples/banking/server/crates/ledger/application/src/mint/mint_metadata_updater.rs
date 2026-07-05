use super::{MintMetadataUpdateRequest, MintMetadataUpdaterError};

#[allow(async_fn_in_trait)]
pub trait MintMetadataUpdater: Send + Sync {
    async fn update(
        &self,
        request: MintMetadataUpdateRequest,
    ) -> Result<(), MintMetadataUpdaterError>;
}
