use crate::http::cache::{
    HttpResourceCacheData, HttpResourceCacheEntry, HttpResourceCacheUrl, HttpResourceEntityTag,
    HttpResourceExpiresAt, HttpResourceFetchedAt, HttpResourceLastModifiedAt,
};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use url::Url;
use uuid::Uuid;

use super::pg_http_resource_cache_row_error::PgHttpResourceCacheRowError;

#[derive(Clone, Debug, FromRow)]
pub struct PgHttpResourceCacheRow {
    pub id: Uuid,
    pub url: String,
    pub data: Vec<u8>,
    pub fetched_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub entity_tag: Option<String>,
}

impl PgHttpResourceCacheRow {
    pub fn try_into_entry(self) -> Result<HttpResourceCacheEntry, PgHttpResourceCacheRowError> {
        let url = Url::parse(&self.url).map_err(|_| PgHttpResourceCacheRowError::InvalidUrl)?;
        let cache_url = HttpResourceCacheUrl::new(url);

        let entry = HttpResourceCacheEntry::new(
            cache_url,
            HttpResourceCacheData::new(self.data),
            HttpResourceFetchedAt::new(self.fetched_at),
            HttpResourceExpiresAt::new(self.expires_at),
        )
        .with_last_modified_at(self.last_modified_at.map(HttpResourceLastModifiedAt::new))
        .with_entity_tag(self.entity_tag.map(HttpResourceEntityTag::new));

        Ok(entry)
    }
}
