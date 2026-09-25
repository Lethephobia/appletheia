use thiserror::Error;

#[derive(Debug, Error)]
pub enum PubsubTopicNameError {
    #[error("topic name is empty")]
    Empty,
}
