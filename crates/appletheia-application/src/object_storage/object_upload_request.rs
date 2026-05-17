use serde::{Deserialize, Serialize};

use super::{
    ObjectBucketName, ObjectChecksum, ObjectContentLength, ObjectContentType, ObjectName,
    ObjectUploadBody,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectUploadRequest {
    bucket_name: ObjectBucketName,
    object_name: ObjectName,
    content_type: ObjectContentType,
    body: ObjectUploadBody,
    checksum: Option<ObjectChecksum>,
}

impl ObjectUploadRequest {
    pub fn new(
        bucket_name: ObjectBucketName,
        object_name: ObjectName,
        content_type: ObjectContentType,
        body: ObjectUploadBody,
    ) -> Self {
        Self {
            bucket_name,
            object_name,
            content_type,
            body,
            checksum: None,
        }
    }

    pub fn with_checksum(mut self, checksum: ObjectChecksum) -> Self {
        self.checksum = Some(checksum);
        self
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

    pub fn content_length(&self) -> ObjectContentLength {
        self.body.content_length()
    }

    pub fn body(&self) -> &ObjectUploadBody {
        &self.body
    }

    pub fn checksum(&self) -> Option<&ObjectChecksum> {
        self.checksum.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        ObjectBucketName,
        ObjectName,
        ObjectContentType,
        ObjectUploadBody,
        Option<ObjectChecksum>,
    ) {
        (
            self.bucket_name,
            self.object_name,
            self.content_type,
            self.body,
            self.checksum,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ObjectBucketName, ObjectContentType, ObjectName, ObjectUploadBody, ObjectUploadRequest,
    };

    #[test]
    fn content_length_is_derived_from_body() {
        let request = ObjectUploadRequest::new(
            ObjectBucketName::new("bucket".to_owned()).expect("bucket should be valid"),
            ObjectName::new("object.json".to_owned()).expect("object name should be valid"),
            ObjectContentType::json(),
            ObjectUploadBody::new(br#"{"ok":true}"#.to_vec()),
        );

        assert_eq!(request.content_length().value(), 11);
    }
}
