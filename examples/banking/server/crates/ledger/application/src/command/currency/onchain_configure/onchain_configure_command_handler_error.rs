use thiserror::Error;

use crate::config::OnchainConfigurerError;

/// Represents errors returned while configuring the on-chain ledger backend.
#[derive(Debug, Error)]
pub enum OnchainConfigureCommandHandlerError {
    #[error("on-chain configurer failed")]
    OnchainConfigurer(#[from] OnchainConfigurerError),
}
