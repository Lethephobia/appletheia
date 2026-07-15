use serde::{Deserialize, Serialize};

use super::ObjectContentLength;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectUploadBody(Vec<u8>);

impl ObjectUploadBody {
    pub fn new(value: Vec<u8>) -> Self {
        Self(value)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn content_length(&self) -> ObjectContentLength {
        ObjectContentLength::new(self.0.len() as u64)
    }
}

impl AsRef<[u8]> for ObjectUploadBody {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl From<Vec<u8>> for ObjectUploadBody {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl From<ObjectUploadBody> for Vec<u8> {
    fn from(value: ObjectUploadBody) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectUploadBody;

    #[test]
    fn content_length_returns_body_length() {
        let body = ObjectUploadBody::new(vec![1, 2, 3]);

        assert_eq!(body.content_length().value(), 3);
    }
}
