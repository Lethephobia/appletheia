use super::{MintMetadataPublishRequest, MintMetadataPublisherError, MintMetadataUri};

/// Publishes an off-chain mint metadata document and returns its URI.
#[allow(async_fn_in_trait)]
pub trait MintMetadataPublisher: Send + Sync {
    async fn publish(
        &self,
        request: MintMetadataPublishRequest,
    ) -> Result<MintMetadataUri, MintMetadataPublisherError>;
}
