use thiserror::Error;

#[derive(Debug, Error)]
pub enum TopicIdError {
    #[error("topic id is empty")]
    Empty,
}
