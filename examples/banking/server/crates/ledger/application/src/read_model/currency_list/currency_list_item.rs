use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

use super::{CurrencyListItemOwner, CurrencyListItemStatus};

/// Read model for one public currency list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListItem {
    pub currency_id: CurrencyId,
    pub owner: CurrencyListItemOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub supply: CurrencyAmount,
    pub status: CurrencyListItemStatus,
    pub created_at: EventOccurredAt,
}
