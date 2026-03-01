use appletheia_application::authentication::oidc::{
    OidcIssuerUrl, OidcProviderMetadata, OidcProviderMetadataSource,
    OidcProviderMetadataSourceError,
};
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

use super::OidcProviderMetadataBody;
use super::http_oidc_provider_metadata_source_config::HttpOidcProviderMetadataSourceConfig;

#[derive(Debug, Clone)]
pub struct HttpOidcProviderMetadataSource<CS>
where
    CS: HttpResourceCacheStore,
{
    config: HttpOidcProviderMetadataSourceConfig,
    cache_store: CS,
    client: reqwest::Client,
}

impl<CS> HttpOidcProviderMetadataSource<CS>
where
    CS: HttpResourceCacheStore,
{
    pub fn new(
        config: HttpOidcProviderMetadataSourceConfig,
        cache_store: CS,
        client: reqwest::Client,
    ) -> Self {
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

    fn parse_and_validate(
        expected_issuer_url: &OidcIssuerUrl,
        bytes: &[u8],
    ) -> Result<OidcProviderMetadata, OidcProviderMetadataSourceError> {
        let body = OidcProviderMetadataBody::try_from_json_bytes(bytes)
            .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?;
        let provider_metadata = body.into_provider_metadata();

        if &provider_metadata.issuer_url != expected_issuer_url {
            return Err(OidcProviderMetadataSourceError::IssuerMismatch {
                expected: Box::new(expected_issuer_url.clone()),
                actual: Box::new(provider_metadata.issuer_url),
            });
        }

        Ok(provider_metadata)
    }
}

impl<CS> OidcProviderMetadataSource for HttpOidcProviderMetadataSource<CS>
where
    CS: HttpResourceCacheStore,
{
    async fn read_provider_metadata(
        &self,
        issuer_url: &OidcIssuerUrl,
    ) -> Result<OidcProviderMetadata, OidcProviderMetadataSourceError> {
        let discovery_url = issuer_url.discovery_url();
        let cache_url = HttpResourceCacheUrl::new(discovery_url.value().clone());

        let cached = self
            .cache_store
            .read_by_url(&cache_url)
            .await
            .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?;

        let now = Utc::now();
        if let Some(entry) = cached.as_ref()
            && entry.expires_at().value() >= now
        {
            return Self::parse_and_validate(issuer_url, entry.data().value());
        }

        let mut request_builder = self.client.get(discovery_url.value().clone());

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
            .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?;

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
                    .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?
                    .to_vec();

                let expires_at = self.resolve_expires_at(now, computed_expires_at);
                let entry = HttpResourceCacheEntry::new(
                    cache_url,
                    HttpResourceCacheData::new(bytes.clone()),
                    HttpResourceFetchedAt::new(now),
                    HttpResourceExpiresAt::new(expires_at),
                )
                .with_entity_tag(entity_tag)
                .with_last_modified_at(last_modified_at);

                self.cache_store
                    .upsert(&entry)
                    .await
                    .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?;

                Self::parse_and_validate(issuer_url, &bytes)
            }
            StatusCode::NOT_MODIFIED => {
                let Some(cached) = cached else {
                    return Err(OidcProviderMetadataSourceError::Backend(Box::new(
                        std::io::Error::other("received 304 without cache"),
                    )));
                };

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
                    .map_err(|source| OidcProviderMetadataSourceError::Backend(Box::new(source)))?;

                Self::parse_and_validate(issuer_url, entry.data().value())
            }
            status => Err(OidcProviderMetadataSourceError::Backend(Box::new(
                std::io::Error::other(format!("unexpected status code: {status}")),
            ))),
        }
    }
}
