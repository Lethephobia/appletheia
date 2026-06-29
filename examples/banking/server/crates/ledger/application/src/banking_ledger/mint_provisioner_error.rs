use thiserror::Error;

/// Represents errors returned while provisioning an on-chain mint.
#[derive(Debug, Error)]
pub enum MintProvisionerError {
    #[error("mint provisioner backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
