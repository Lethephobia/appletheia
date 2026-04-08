use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceError};
use thiserror::Error;

/// Represents errors returned while completing a currency issuance.
#[derive(Debug, Error)]
pub enum CurrencyIssuanceCompleteCommandHandlerError {
    #[error("currency issuance repository failed")]
    CurrencyIssuanceRepository(#[from] RepositoryError<CurrencyIssuance>),

    #[error("currency issuance aggregate failed")]
    CurrencyIssuance(#[from] CurrencyIssuanceError),

    #[error("currency issuance was not found")]
    CurrencyIssuanceNotFound,
}
