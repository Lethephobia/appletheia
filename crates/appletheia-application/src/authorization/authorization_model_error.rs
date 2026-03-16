use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationModelError {
    #[error("authorization model backend error: {0}")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl AuthorizationModelError {
    pub fn backend<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::Backend(Box::new(error))
    }
}
