use appletheia_application::authentication::oidc::OidcJwksUri;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use reqwest::header::{
    CACHE_CONTROL, ETAG, EXPIRES, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED,
};

use crate::http::cache::{
    HttpResourceCacheData, HttpResourceCacheEntry, HttpResourceCacheStore, HttpResourceCacheUrl,
    HttpResourceEntityTag, HttpResourceExpiresAt, HttpResourceFetchedAt,
    HttpResourceLastModifiedAt,
};
use crate::jwt::{Jwks, JwksSource, JwksSourceError};

use super::http_jwks_source_config::HttpJwksSourceConfig;

#[derive(Debug, Clone)]
pub struct HttpJwksSource<CS>
where
    CS: HttpResourceCacheStore,
{
    config: HttpJwksSourceConfig,
    cache_store: CS,
    client: reqwest::Client,
}

impl<CS> HttpJwksSource<CS>
where
    CS: HttpResourceCacheStore,
{
    pub fn new(config: HttpJwksSourceConfig, cache_store: CS, client: reqwest::Client) -> Self {
        Self {
            config,
            cache_store,
            client,
        }
    }

    fn resolve_expires_at(
        &self,
        now: DateTime<Utc>,
        computed_expires_at: Option<DateTime<Utc>>,
    ) -> DateTime<Utc> {
        computed_expires_at.unwrap_or(now + self.config.fallback_ttl().value())
    }

    fn parse_jwks(bytes: &[u8]) -> Result<Jwks, JwksSourceError> {
        serde_json::from_slice(bytes).map_err(|source| JwksSourceError::InvalidJwks {
            source: Box::new(source),
        })
    }
}

impl<CS> JwksSource for HttpJwksSource<CS>
where
    CS: HttpResourceCacheStore,
{
    async fn read_jwks(&self, jwks_uri: &OidcJwksUri) -> Result<Jwks, JwksSourceError> {
        let cache_url = HttpResourceCacheUrl::new(jwks_uri.value().clone());

        let cached = self
            .cache_store
            .read_by_url(&cache_url)
            .await
            .map_err(|source| JwksSourceError::Backend(Box::new(source)))?;

        let now = Utc::now();
        if let Some(entry) = cached.as_ref()
            && entry.expires_at().value() >= now
        {
            return Self::parse_jwks(entry.data().value());
        }

        let mut request_builder = self.client.get(jwks_uri.value().clone());

        if let Some(entity_tag) = cached.as_ref().and_then(|entry| entry.entity_tag()) {
            request_builder = request_builder.header(IF_NONE_MATCH, entity_tag.value());
        }

        if let Some(last_modified_at) = cached.as_ref().and_then(|entry| entry.last_modified_at())
            && let Some(formatted) = last_modified_at.to_http_date_string()
        {
            request_builder = request_builder.header(IF_MODIFIED_SINCE, formatted);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|source| JwksSourceError::Backend(Box::new(source)))?;

        let headers = response.headers().clone();
        let computed_expires_at = HttpResourceExpiresAt::from_cache_headers(
            now,
            headers.get(CACHE_CONTROL).and_then(|v| v.to_str().ok()),
            headers.get(EXPIRES).and_then(|v| v.to_str().ok()),
        )
        .map(|value| value.value());
        let entity_tag = headers
            .get(ETAG)
            .and_then(|v| v.to_str().ok())
            .map(HttpResourceEntityTag::from_header_str);
        let last_modified_at = headers
            .get(LAST_MODIFIED)
            .and_then(|v| v.to_str().ok())
            .and_then(HttpResourceLastModifiedAt::from_http_date_str);

        match response.status() {
            StatusCode::OK => {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|source| JwksSourceError::Backend(Box::new(source)))?
                    .to_vec();

                let jwks = Self::parse_jwks(&bytes)?;

                let expires_at = self.resolve_expires_at(now, computed_expires_at);
                let entry = HttpResourceCacheEntry::new(
                    cache_url,
                    HttpResourceCacheData::new(bytes),
                    HttpResourceFetchedAt::new(now),
                    HttpResourceExpiresAt::new(expires_at),
                )
                .with_entity_tag(entity_tag)
                .with_last_modified_at(last_modified_at);

                self.cache_store
                    .upsert(&entry)
                    .await
                    .map_err(|source| JwksSourceError::Backend(Box::new(source)))?;

                Ok(jwks)
            }
            StatusCode::NOT_MODIFIED => {
                let Some(cached) = cached else {
                    return Err(JwksSourceError::Backend(Box::new(std::io::Error::other(
                        "received 304 without cache",
                    ))));
                };

                let jwks = Self::parse_jwks(cached.data().value())?;

                let expires_at = self.resolve_expires_at(now, computed_expires_at);
                let resolved_entity_tag = entity_tag.or_else(|| cached.entity_tag().cloned());
                let resolved_last_modified_at =
                    last_modified_at.or_else(|| cached.last_modified_at());

                let entry = HttpResourceCacheEntry::new(
                    cache_url,
                    HttpResourceCacheData::new(cached.data().value().to_vec()),
                    HttpResourceFetchedAt::new(now),
                    HttpResourceExpiresAt::new(expires_at),
                )
                .with_entity_tag(resolved_entity_tag)
                .with_last_modified_at(resolved_last_modified_at);

                self.cache_store
                    .upsert(&entry)
                    .await
                    .map_err(|source| JwksSourceError::Backend(Box::new(source)))?;

                Ok(jwks)
            }
            status => Err(JwksSourceError::Backend(Box::new(std::io::Error::other(
                format!("unexpected status code: {status}"),
            )))),
        }
    }
}
