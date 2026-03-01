use super::{HttpResourceCacheEntry, HttpResourceCacheStoreError, HttpResourceCacheUrl};

#[allow(async_fn_in_trait)]
pub trait HttpResourceCacheStore: Send + Sync {
    async fn read_by_url(
        &self,
        url: &HttpResourceCacheUrl,
    ) -> Result<Option<HttpResourceCacheEntry>, HttpResourceCacheStoreError>;

    async fn upsert(
        &self,
        entry: &HttpResourceCacheEntry,
    ) -> Result<(), HttpResourceCacheStoreError>;
}
