use super::{MintAccountMetadataUpdateRequest, MintAccountMetadataUpdaterError};

/// Updates metadata for an existing on-chain mint account.
#[allow(async_fn_in_trait)]
pub trait MintAccountMetadataUpdater: Send + Sync {
    async fn update(
        &self,
        request: MintAccountMetadataUpdateRequest,
    ) -> Result<(), MintAccountMetadataUpdaterError>;
}
