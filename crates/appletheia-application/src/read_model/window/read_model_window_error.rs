use super::ReadModelWindowCursorError;

#[derive(Debug, thiserror::Error)]
pub enum ReadModelWindowError {
    #[error(transparent)]
    Cursor(#[from] ReadModelWindowCursorError),
}
