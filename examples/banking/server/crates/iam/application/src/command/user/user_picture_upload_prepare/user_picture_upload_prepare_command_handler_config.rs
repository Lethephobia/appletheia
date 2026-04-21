use std::collections::BTreeSet;

use appletheia::application::object_storage::{
    ObjectBucketName, ObjectContentLength, ObjectContentType, ObjectUploadExpiresIn,
};

/// Configuration for `UserPictureUploadPrepareCommandHandler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPictureUploadPrepareCommandHandlerConfig {
    bucket_name: ObjectBucketName,
    expires_in: ObjectUploadExpiresIn,
    max_content_length: ObjectContentLength,
    allowed_content_types: BTreeSet<ObjectContentType>,
}

impl UserPictureUploadPrepareCommandHandlerConfig {
    /// Creates a new `UserPictureUploadPrepareCommandHandler` configuration.
    pub fn new(
        bucket_name: ObjectBucketName,
        expires_in: ObjectUploadExpiresIn,
        max_content_length: ObjectContentLength,
        allowed_content_types: BTreeSet<ObjectContentType>,
    ) -> Self {
        Self {
            bucket_name,
            expires_in,
            max_content_length,
            allowed_content_types,
        }
    }

    /// Returns the target object storage bucket name.
    pub fn bucket_name(&self) -> &ObjectBucketName {
        &self.bucket_name
    }

    /// Returns the signed upload expiration duration.
    pub fn expires_in(&self) -> ObjectUploadExpiresIn {
        self.expires_in
    }

    /// Returns the maximum allowed content length.
    pub fn max_content_length(&self) -> ObjectContentLength {
        self.max_content_length
    }

    /// Returns the allowed content types.
    pub fn allowed_content_types(&self) -> &BTreeSet<ObjectContentType> {
        &self.allowed_content_types
    }
}
