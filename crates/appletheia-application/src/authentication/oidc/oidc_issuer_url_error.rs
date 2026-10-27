use thiserror::Error;

#[derive(Debug, Error)]
pub enum OidcIssuerUrlError {
    #[error("issuer URL cannot be used as a base URL")]
    CannotBeABase,
}
