use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObjectBucketNameError {
    #[error("object storage bucket name is empty")]
    Empty,
}
