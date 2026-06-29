use banking_ledger_domain::currency::{
    CurrencyMintAccountAddressError, CurrencyPoolTokenAccountAddressError,
};
use thiserror::Error;

/// Describes why a mint initialize receipt cannot be mapped into the domain.
#[derive(Debug, Error)]
pub enum MintProvisionReceiptError {
    #[error("mint account address is invalid")]
    MintAccountAddress(#[from] CurrencyMintAccountAddressError),

    #[error("pool token account address is invalid")]
    PoolTokenAccountAddress(#[from] CurrencyPoolTokenAccountAddressError),
}
