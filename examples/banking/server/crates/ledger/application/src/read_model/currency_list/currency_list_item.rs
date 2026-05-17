use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName,
    CurrencySymbol,
};

use super::{CurrencyListItemOwner, CurrencyListItemStatus};
use crate::read_model::ReadModelObservation;

/// Read model for one public currency list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListItem {
    pub currency_id: CurrencyId,
    pub owner: CurrencyListItemOwner,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub image: Option<CurrencyImageRef>,
    pub supply: CurrencyAmount,
    pub status: CurrencyListItemStatus,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl CurrencyListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.owner.observed_event_ids()),
        )
    }
}
