use super::{
    HttpResourceCacheData, HttpResourceCacheUrl, HttpResourceEntityTag, HttpResourceExpiresAt,
    HttpResourceFetchedAt, HttpResourceLastModifiedAt,
};

#[derive(Clone, Debug)]
pub struct HttpResourceCacheEntry {
    url: HttpResourceCacheUrl,
    data: HttpResourceCacheData,
    fetched_at: HttpResourceFetchedAt,
    expires_at: HttpResourceExpiresAt,
    last_modified_at: Option<HttpResourceLastModifiedAt>,
    entity_tag: Option<HttpResourceEntityTag>,
}

impl HttpResourceCacheEntry {
    pub fn new(
        url: HttpResourceCacheUrl,
        data: HttpResourceCacheData,
        fetched_at: HttpResourceFetchedAt,
        expires_at: HttpResourceExpiresAt,
    ) -> Self {
        Self {
            url,
            data,
            fetched_at,
            expires_at,
            last_modified_at: None,
            entity_tag: None,
        }
    }

    pub fn with_last_modified_at(mut self, value: Option<HttpResourceLastModifiedAt>) -> Self {
        self.last_modified_at = value;
        self
    }

    pub fn with_entity_tag(mut self, value: Option<HttpResourceEntityTag>) -> Self {
        self.entity_tag = value;
        self
    }

    pub fn url(&self) -> &HttpResourceCacheUrl {
        &self.url
    }

    pub fn data(&self) -> &HttpResourceCacheData {
        &self.data
    }

    pub fn fetched_at(&self) -> HttpResourceFetchedAt {
        self.fetched_at
    }

    pub fn expires_at(&self) -> HttpResourceExpiresAt {
        self.expires_at
    }

    pub fn last_modified_at(&self) -> Option<HttpResourceLastModifiedAt> {
        self.last_modified_at
    }

    pub fn entity_tag(&self) -> Option<&HttpResourceEntityTag> {
        self.entity_tag.as_ref()
    }
}
