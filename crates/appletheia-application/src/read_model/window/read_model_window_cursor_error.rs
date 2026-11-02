#[derive(Debug, thiserror::Error)]
pub enum ReadModelWindowCursorError {
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
