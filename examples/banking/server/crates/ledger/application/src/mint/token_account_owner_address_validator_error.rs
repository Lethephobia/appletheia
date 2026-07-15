use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenAccountOwnerAddressValidatorError {
    #[error("token account owner address is invalid")]
    InvalidAddress,

    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
