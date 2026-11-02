#[derive(Debug, thiserror::Error)]
pub enum ReadModelCursorError {
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
