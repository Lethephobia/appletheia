use thiserror::Error;

/// Describes why an OIDC return destination is invalid.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum OidcReturnToError {
    #[error("OIDC return destination must be an application-local absolute path")]
    NotApplicationLocal,
}
