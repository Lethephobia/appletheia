#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResourceCacheData(Vec<u8>);

impl HttpResourceCacheData {
    pub fn new(value: Vec<u8>) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &[u8] {
        &self.0
    }
}
