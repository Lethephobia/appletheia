/// Describes why a mint metadata URI is invalid.
#[derive(Debug, thiserror::Error)]
pub enum MintMetadataUriError {
    #[error("mint metadata URI is invalid")]
    Parse(#[from] url::ParseError),
}
