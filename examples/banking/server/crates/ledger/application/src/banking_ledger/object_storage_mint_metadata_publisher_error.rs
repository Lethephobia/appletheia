use appletheia::application::object_storage::{ObjectNameError, ObjectUploaderError};

use super::MintMetadataPublicBaseUrlError;

#[derive(Debug, thiserror::Error)]
pub enum ObjectStorageMintMetadataPublisherError {
    #[error("mint metadata JSON serialization failed")]
    Serialize(#[source] serde_json::Error),
    #[error("mint metadata object name is invalid")]
    ObjectName(#[from] ObjectNameError),
    #[error("mint metadata public URL is invalid")]
    PublicBaseUrl(#[from] MintMetadataPublicBaseUrlError),
    #[error("mint metadata object upload failed")]
    ObjectUploader(#[from] ObjectUploaderError),
}
