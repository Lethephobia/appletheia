use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpResourceCacheStoreError {
    #[error("backend error")]
    Backend(#[source] Box<dyn Error + Send + Sync>),
}
