use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectNameError {
    #[error("object storage object name is empty")]
    Empty,
}
