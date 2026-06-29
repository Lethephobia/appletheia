#[derive(Debug, thiserror::Error)]
pub enum MintMetadataObjectNameError {
    #[error("mint metadata object name is empty")]
    Empty,
}
