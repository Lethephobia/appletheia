use banking_ledger_domain::currency::{
    CurrencyMintAccountAddressError, CurrencyPoolTokenAccountAddressError,
    CurrencyTokenProgramIdError,
};
use thiserror::Error;

/// Describes why a mint account creation receipt cannot be mapped into the domain.
#[derive(Debug, Error)]
pub enum MintAccountCreateReceiptError {
    #[error("mint account address is invalid")]
    MintAccountAddress(#[from] CurrencyMintAccountAddressError),

    #[error("token program ID is invalid")]
    TokenProgramId(#[from] CurrencyTokenProgramIdError),

    #[error("pool token account address is invalid")]
    PoolTokenAccountAddress(#[from] CurrencyPoolTokenAccountAddressError),
}
