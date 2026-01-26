use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerGroupError {
    #[error("consumer group is empty")]
    Empty,
}

