use serde::{Deserialize, Serialize};

use super::{ObjectUploadHeaderName, ObjectUploadHeaderValue};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectUploadHeader {
    name: ObjectUploadHeaderName,
    value: ObjectUploadHeaderValue,
}

impl ObjectUploadHeader {
    pub fn new(name: ObjectUploadHeaderName, value: ObjectUploadHeaderValue) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &ObjectUploadHeaderName {
        &self.name
    }

    pub fn value(&self) -> &ObjectUploadHeaderValue {
        &self.value
    }
}
