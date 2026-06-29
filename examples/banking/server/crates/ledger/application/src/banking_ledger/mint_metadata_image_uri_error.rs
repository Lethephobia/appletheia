/// Describes why a mint metadata image URI is invalid.
#[derive(Debug, thiserror::Error)]
pub enum MintMetadataImageUriError {
    #[error("mint metadata image URI is invalid")]
    Parse(#[from] url::ParseError),
}
