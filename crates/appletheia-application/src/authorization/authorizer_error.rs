use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizerError {
    #[error("principal is unavailable in request context")]
    PrincipalUnavailable,

    #[error("unauthenticated")]
    Unauthenticated,

    #[error("tenant_id is required for authorization")]
    TenantRequired,

    #[error("resource is required for authorization")]
    ResourceRequired,

    #[error("forbidden")]
    Forbidden,

    #[error("authorization backend error: {0}")]
    Backend(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl AuthorizerError {
    pub fn backend<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::Backend(Box::new(error))
    }
}
