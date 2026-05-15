use appletheia::application::event::EventSequence;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencySymbol,
};

use super::CurrencyListItemStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListCurrencyUpsert {
    pub id: CurrencyId,
    pub owner: CurrencyOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub supply: CurrencyAmount,
    pub status: CurrencyListItemStatus,
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}
