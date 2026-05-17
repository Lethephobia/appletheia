#[derive(Debug, thiserror::Error)]
pub enum MintMetadataPublicBaseUrlError {
    #[error("mint metadata public base URL is invalid")]
    Parse(#[from] url::ParseError),
    #[error("mint metadata public base URL cannot be used as a base URL")]
    InvalidBaseUrl,
}
