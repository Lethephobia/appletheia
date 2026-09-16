use chrono::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventTimeError {
    #[error(transparent)]
    Parse(#[from] ParseError),
}
