use appletheia::application::object_storage::ObjectBucketName;

use super::MintMetadataPublicBaseUrl;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectStorageMintMetadataPublisherConfig {
    bucket_name: ObjectBucketName,
    metadata_public_base_url: MintMetadataPublicBaseUrl,
}

impl ObjectStorageMintMetadataPublisherConfig {
    pub fn new(
        bucket_name: ObjectBucketName,
        metadata_public_base_url: MintMetadataPublicBaseUrl,
    ) -> Self {
        Self {
            bucket_name,
            metadata_public_base_url,
        }
    }

    pub fn bucket_name(&self) -> &ObjectBucketName {
        &self.bucket_name
    }

    pub fn metadata_public_base_url(&self) -> &MintMetadataPublicBaseUrl {
        &self.metadata_public_base_url
    }
}
