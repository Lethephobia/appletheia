use thiserror::Error;

#[derive(Debug, Error)]
pub enum MintProvisionerError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
