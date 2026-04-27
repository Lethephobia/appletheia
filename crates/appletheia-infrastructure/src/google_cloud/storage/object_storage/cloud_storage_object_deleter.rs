use appletheia_application::{
    ObjectBucketName, ObjectDeleteRequest, ObjectDeleter, ObjectDeleterError,
};
use google_cloud_storage::client::StorageControl;

use super::CloudStorageObjectDeleterError;

#[derive(Clone, Debug)]
pub struct CloudStorageObjectDeleter {
    client: StorageControl,
}

impl CloudStorageObjectDeleter {
    pub fn new(client: StorageControl) -> Self {
        Self { client }
    }

    fn bucket_resource_name(bucket_name: &ObjectBucketName) -> String {
        format!("projects/_/buckets/{}", bucket_name.as_str())
    }
}

impl ObjectDeleter for CloudStorageObjectDeleter {
    async fn delete(&self, request: ObjectDeleteRequest) -> Result<(), ObjectDeleterError> {
        self.client
            .delete_object()
            .set_bucket(Self::bucket_resource_name(request.bucket_name()))
            .set_object(request.object_name().as_str())
            .send()
            .await
            .map_err(CloudStorageObjectDeleterError::Delete)
            .map_err(|error| ObjectDeleterError::Backend(Box::new(error)))
    }
}
