use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

use crate::onchain::{
    MintAccountCreateReceiptError, MintAccountCreatorError, MintAccountSeedError,
    MintMetadataPublisherError, MintMetadataUriError,
};

/// Represents errors returned while creating a currency mint account.
#[derive(Debug, Error)]
pub enum CurrencyMintAccountCreateCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint account seed is invalid")]
    MintAccountSeed(#[from] MintAccountSeedError),

    #[error("mint metadata URI is invalid")]
    MintMetadataUri(#[from] MintMetadataUriError),

    #[error("mint metadata publisher failed")]
    MintMetadataPublisher(#[from] MintMetadataPublisherError),

    #[error("mint account creator failed")]
    MintAccountCreator(#[from] MintAccountCreatorError),

    #[error("mint account creation receipt is invalid")]
    MintAccountCreateReceipt(#[from] MintAccountCreateReceiptError),

    #[error("currency was not found")]
    CurrencyNotFound,
}
