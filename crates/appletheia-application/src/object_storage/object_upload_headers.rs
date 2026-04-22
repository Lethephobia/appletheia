use serde::{Deserialize, Serialize};

use super::ObjectUploadHeader;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectUploadHeaders(Vec<ObjectUploadHeader>);

impl ObjectUploadHeaders {
    pub fn as_slice(&self) -> &[ObjectUploadHeader] {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &ObjectUploadHeader> {
        self.0.iter()
    }
}

impl From<Vec<ObjectUploadHeader>> for ObjectUploadHeaders {
    fn from(value: Vec<ObjectUploadHeader>) -> Self {
        Self(value)
    }
}

impl FromIterator<ObjectUploadHeader> for ObjectUploadHeaders {
    fn from_iter<T: IntoIterator<Item = ObjectUploadHeader>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for ObjectUploadHeaders {
    type Item = ObjectUploadHeader;
    type IntoIter = std::vec::IntoIter<ObjectUploadHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
