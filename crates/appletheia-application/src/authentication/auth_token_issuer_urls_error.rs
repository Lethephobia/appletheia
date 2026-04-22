use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum AuthTokenIssuerUrlsError {
    #[error("issuer urls must not be empty")]
    Empty,
}
