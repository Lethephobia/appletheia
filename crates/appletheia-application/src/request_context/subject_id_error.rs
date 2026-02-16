use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectIdError {
    #[error("subject id must be a valid uuid: {value}")]
    InvalidUuid {
        value: String,
        #[source]
        source: uuid::Error,
    },
}
