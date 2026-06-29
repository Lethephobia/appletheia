#[derive(Debug, thiserror::Error)]
pub enum MintMetadataImagePublicBaseUrlError {
    #[error("mint metadata image public base URL is invalid")]
    Parse(#[from] url::ParseError),
    #[error("mint metadata image public base URL cannot be used as a base URL")]
    InvalidBaseUrl,
}
