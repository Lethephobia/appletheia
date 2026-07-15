use appletheia::application::object_storage::ObjectBucketName;
use url::Url;

/// Configuration for `ObjectStorageMintMetadataPublisher`.
pub struct ObjectStorageMintMetadataPublisherConfig {
    bucket_name: ObjectBucketName,
    metadata_public_base_url: Url,
    image_public_base_url: Url,
}

impl ObjectStorageMintMetadataPublisherConfig {
    pub fn new(
        bucket_name: ObjectBucketName,
        metadata_public_base_url: Url,
        image_public_base_url: Url,
    ) -> Self {
        Self {
            bucket_name,
            metadata_public_base_url,
            image_public_base_url,
        }
    }

    pub fn bucket_name(&self) -> &ObjectBucketName {
        &self.bucket_name
    }

    pub fn metadata_public_base_url(&self) -> &Url {
        &self.metadata_public_base_url
    }

    pub fn image_public_base_url(&self) -> &Url {
        &self.image_public_base_url
    }
}
