use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectContentLength(u64);

impl ObjectContentLength {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<ObjectContentLength> for u64 {
    fn from(value: ObjectContentLength) -> Self {
        value.0
    }
}
