use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef,
    CurrencyMintAccountAddress, CurrencyName, CurrencyOwner, CurrencySymbol,
};

use super::CurrencyListItemStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListCurrencyUpsert {
    pub id: CurrencyId,
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub image: Option<CurrencyImageRef>,
    pub mint_account_address: Option<CurrencyMintAccountAddress>,
    pub supply: CurrencyAmount,
    pub status: CurrencyListItemStatus,
}
