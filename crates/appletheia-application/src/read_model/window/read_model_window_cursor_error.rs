use super::ReadModelResourceWindowCursorError;

#[derive(Debug, thiserror::Error)]
pub enum ReadModelWindowCursorError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    ResourceWindowCursor(#[from] ReadModelResourceWindowCursorError),
}
