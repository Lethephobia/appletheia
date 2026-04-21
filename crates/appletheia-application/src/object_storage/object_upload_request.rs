use serde::{Deserialize, Serialize};

use super::{
    ObjectBucketName, ObjectChecksum, ObjectContentLength, ObjectContentType, ObjectName,
    ObjectUploadExpiresIn, ObjectUploadMethod,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectUploadRequest {
    method: ObjectUploadMethod,
    bucket_name: ObjectBucketName,
    object_name: ObjectName,
    content_type: ObjectContentType,
    content_length: Option<ObjectContentLength>,
    checksum: Option<ObjectChecksum>,
    expires_in: ObjectUploadExpiresIn,
}

impl ObjectUploadRequest {
    pub fn new(
        bucket_name: ObjectBucketName,
        object_name: ObjectName,
        content_type: ObjectContentType,
        expires_in: ObjectUploadExpiresIn,
    ) -> Self {
        Self {
            method: ObjectUploadMethod::Put,
            bucket_name,
            object_name,
            content_type,
            content_length: None,
            checksum: None,
            expires_in,
        }
    }

    pub fn with_content_length(mut self, content_length: ObjectContentLength) -> Self {
        self.content_length = Some(content_length);
        self
    }

    pub fn with_checksum(mut self, checksum: ObjectChecksum) -> Self {
        self.checksum = Some(checksum);
        self
    }

    pub fn method(&self) -> ObjectUploadMethod {
        self.method
    }

    pub fn bucket_name(&self) -> &ObjectBucketName {
        &self.bucket_name
    }

    pub fn object_name(&self) -> &ObjectName {
        &self.object_name
    }

    pub fn content_type(&self) -> &ObjectContentType {
        &self.content_type
    }

    pub fn content_length(&self) -> Option<ObjectContentLength> {
        self.content_length
    }

    pub fn checksum(&self) -> Option<&ObjectChecksum> {
        self.checksum.as_ref()
    }

    pub fn expires_in(&self) -> ObjectUploadExpiresIn {
        self.expires_in
    }
}
