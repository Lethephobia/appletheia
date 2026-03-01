use crate::http::cache::HttpResourceCacheFallbackTtl;

#[derive(Clone, Copy, Debug)]
pub struct HttpJwksSourceConfig {
    fallback_ttl: HttpResourceCacheFallbackTtl,
}

impl HttpJwksSourceConfig {
    pub fn new(fallback_ttl: HttpResourceCacheFallbackTtl) -> Self {
        Self { fallback_ttl }
    }

    pub fn fallback_ttl(&self) -> HttpResourceCacheFallbackTtl {
        self.fallback_ttl
    }
}
