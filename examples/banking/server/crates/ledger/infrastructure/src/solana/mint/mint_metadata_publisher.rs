use super::{MintMetadataPublishRequest, MintMetadataPublisherError};

#[allow(async_fn_in_trait)]
pub trait MintMetadataPublisher: Send + Sync {
    async fn publish(
        &self,
        request: MintMetadataPublishRequest,
    ) -> Result<String, MintMetadataPublisherError>;
}
