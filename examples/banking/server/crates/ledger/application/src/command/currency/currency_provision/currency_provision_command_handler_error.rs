use crate::mint::MintProvisionerError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while provisioning a currency.
#[derive(Debug, Error)]
pub enum CurrencyProvisionCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint provisioner failed")]
    MintProvisioner(#[from] MintProvisionerError),
}
