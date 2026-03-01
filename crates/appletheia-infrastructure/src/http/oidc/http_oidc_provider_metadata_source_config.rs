use crate::http::cache::HttpResourceCacheFallbackTtl;

#[derive(Clone, Copy, Debug)]
pub struct HttpOidcProviderMetadataSourceConfig {
    fallback_ttl: HttpResourceCacheFallbackTtl,
}

impl HttpOidcProviderMetadataSourceConfig {
    pub fn new(fallback_ttl: HttpResourceCacheFallbackTtl) -> Self {
        Self { fallback_ttl }
    }

    pub fn fallback_ttl(&self) -> HttpResourceCacheFallbackTtl {
        self.fallback_ttl
    }
}
