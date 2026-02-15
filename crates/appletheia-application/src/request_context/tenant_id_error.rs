use thiserror::Error;

#[derive(Debug, Error)]
pub enum TenantIdError {
    #[error("tenant id is empty")]
    Empty,

    #[error("tenant id is too long (len={len}, max={max})")]
    TooLong { len: usize, max: usize },
}
