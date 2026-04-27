use serde::{Deserialize, Serialize};

use super::{ObjectBucketName, ObjectName};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectDeleteRequest {
    bucket_name: ObjectBucketName,
    object_name: ObjectName,
}

impl ObjectDeleteRequest {
    pub fn new(bucket_name: ObjectBucketName, object_name: ObjectName) -> Self {
        Self {
            bucket_name,
            object_name,
        }
    }

    pub fn bucket_name(&self) -> &ObjectBucketName {
        &self.bucket_name
    }

    pub fn object_name(&self) -> &ObjectName {
        &self.object_name
    }
}
