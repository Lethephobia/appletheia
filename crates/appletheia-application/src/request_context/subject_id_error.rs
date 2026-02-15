use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectIdError {
    #[error("subject id is empty")]
    Empty,

    #[error("subject id is too long (len={len}, max={max})")]
    TooLong { len: usize, max: usize },
}
