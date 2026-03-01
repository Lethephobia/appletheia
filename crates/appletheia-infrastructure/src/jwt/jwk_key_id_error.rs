use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwkKeyIdError {
    #[error("key_id must not be empty")]
    Empty,
}
