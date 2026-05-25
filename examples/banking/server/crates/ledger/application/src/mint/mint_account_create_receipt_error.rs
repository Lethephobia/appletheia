use banking_ledger_domain::currency::{
    CurrencyMintAccountAddressError, CurrencyMintTokenProgramIdError,
    CurrencyPoolAccountAddressError,
};
use thiserror::Error;

/// Describes why a mint account creation receipt cannot be mapped into the domain.
#[derive(Debug, Error)]
pub enum MintAccountCreateReceiptError {
    #[error("mint account address is invalid")]
    Address(#[from] CurrencyMintAccountAddressError),

    #[error("token program ID is invalid")]
    TokenProgramId(#[from] CurrencyMintTokenProgramIdError),

    #[error("pool account address is invalid")]
    PoolAddress(#[from] CurrencyPoolAccountAddressError),
}
