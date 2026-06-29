use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

use crate::banking_ledger::{
    MintIdError, MintMetadataImagePublicBaseUrlError, MintMetadataImageUriError,
    MintMetadataPublisherError, MintProvisionReceiptError, MintProvisionerError,
};

/// Represents errors returned while provisioning a currency.
#[derive(Debug, Error)]
pub enum CurrencyProvisionCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint ID is invalid")]
    MintId(#[from] MintIdError),

    #[error("mint metadata image public base URL is invalid")]
    MintMetadataImagePublicBaseUrl(#[from] MintMetadataImagePublicBaseUrlError),

    #[error("mint metadata image URI is invalid")]
    MintMetadataImageUri(#[from] MintMetadataImageUriError),

    #[error("mint metadata publisher failed")]
    MintMetadataPublisher(#[from] MintMetadataPublisherError),

    #[error("mint provisioner failed")]
    MintProvisioner(#[from] MintProvisionerError),

    #[error("mint provision receipt is invalid")]
    MintProvisionReceipt(#[from] MintProvisionReceiptError),
}
