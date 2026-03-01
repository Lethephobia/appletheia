use chrono::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpResourceCacheFallbackTtl(Duration);

impl HttpResourceCacheFallbackTtl {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}
