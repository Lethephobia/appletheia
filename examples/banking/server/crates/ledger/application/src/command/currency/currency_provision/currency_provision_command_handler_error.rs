use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

use crate::mint::{
    MintAccountCreateReceiptError, MintAccountCreatorError, MintAccountSeedError,
    MintMetadataImagePublicBaseUrlError, MintMetadataImageUriError, MintMetadataPublisherError,
};

/// Represents errors returned while provisioning a currency.
#[derive(Debug, Error)]
pub enum CurrencyProvisionCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint account seed is invalid")]
    MintAccountSeed(#[from] MintAccountSeedError),

    #[error("mint metadata image public base URL is invalid")]
    MintMetadataImagePublicBaseUrl(#[from] MintMetadataImagePublicBaseUrlError),

    #[error("mint metadata image URI is invalid")]
    MintMetadataImageUri(#[from] MintMetadataImageUriError),

    #[error("mint metadata publisher failed")]
    MintMetadataPublisher(#[from] MintMetadataPublisherError),

    #[error("mint account creator failed")]
    MintAccountCreator(#[from] MintAccountCreatorError),

    #[error("mint account creation receipt is invalid")]
    MintAccountCreateReceipt(#[from] MintAccountCreateReceiptError),

    #[error("currency was not found")]
    CurrencyNotFound,
}
