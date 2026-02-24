use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum AuthTokenIssuerUrlError {
    #[error("failed to parse issuer URL")]
    Parse(#[source] ParseError),
}
