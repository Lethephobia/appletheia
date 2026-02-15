use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizerError {
    #[error("principal is unavailable in request context")]
    PrincipalUnavailable,

    #[error("unauthenticated")]
    Unauthenticated,

    #[error("forbidden")]
    Forbidden,
}
