use crate::http::cache::{
    HttpResourceCacheEntry, HttpResourceCacheStore, HttpResourceCacheStoreError,
    HttpResourceCacheUrl,
};
use sqlx::PgPool;
use uuid::Uuid;

use super::pg_http_resource_cache_row::PgHttpResourceCacheRow;

#[derive(Debug)]
pub struct PgHttpResourceCacheStore {
    pool: PgPool,
}

impl PgHttpResourceCacheStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl HttpResourceCacheStore for PgHttpResourceCacheStore {
    async fn read_by_url(
        &self,
        url: &HttpResourceCacheUrl,
    ) -> Result<Option<HttpResourceCacheEntry>, HttpResourceCacheStoreError> {
        let row: Option<PgHttpResourceCacheRow> = sqlx::query_as(
            r#"
            SELECT
              id,
              url,
              data,
              fetched_at,
              expires_at,
              last_modified_at,
              entity_tag
            FROM resource_response_cache
            WHERE url = $1
            "#,
        )
        .bind(url.value().as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|source| HttpResourceCacheStoreError::Backend(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let entry = row
            .try_into_entry()
            .map_err(|source| HttpResourceCacheStoreError::Backend(Box::new(source)))?;

        Ok(Some(entry))
    }

    async fn upsert(
        &self,
        entry: &HttpResourceCacheEntry,
    ) -> Result<(), HttpResourceCacheStoreError> {
        let id_value = Uuid::now_v7();
        let url_value = entry.url().value().as_str();
        let data_value = entry.data().value();
        let fetched_at_value = entry.fetched_at().value();
        let expires_at_value = entry.expires_at().value();
        let last_modified_at_value = entry.last_modified_at().map(|value| value.value());
        let entity_tag_value = entry.entity_tag().map(|value| value.value());

        sqlx::query(
            r#"
            INSERT INTO resource_response_cache (
              id,
              url,
              data,
              fetched_at,
              expires_at,
              last_modified_at,
              entity_tag
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              $6,
              $7
            )
            ON CONFLICT (url)
            DO UPDATE SET
              data = excluded.data,
              fetched_at = excluded.fetched_at,
              expires_at = excluded.expires_at,
              last_modified_at = excluded.last_modified_at,
              entity_tag = excluded.entity_tag
            "#,
        )
        .bind(id_value)
        .bind(url_value)
        .bind(data_value)
        .bind(fetched_at_value)
        .bind(expires_at_value)
        .bind(last_modified_at_value)
        .bind(entity_tag_value)
        .execute(&self.pool)
        .await
        .map_err(|source| HttpResourceCacheStoreError::Backend(Box::new(source)))?;

        Ok(())
    }
}
