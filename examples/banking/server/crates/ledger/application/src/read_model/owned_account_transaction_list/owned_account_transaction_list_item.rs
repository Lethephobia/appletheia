use serde::Serialize;

use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::{
    ChainNetwork, CurrencyAmount, OnchainTransactionId, TokenAddress,
};

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItemCurrency,
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus,
};
use crate::projection::TransactionNote;
use appletheia::application::read_model::ReadModelObservation;

/// Read model for one owned account transaction list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnedAccountTransactionListItem {
    pub transaction_id: OwnedAccountTransactionId,
    pub account_id: AccountId,
    pub currency: OwnedAccountTransactionListItemCurrency,
    pub chain_network: Option<ChainNetwork>,
    pub token_address: Option<TokenAddress>,
    pub onchain_transaction_id: Option<OnchainTransactionId>,
    pub amount: CurrencyAmount,
    pub note: Option<TransactionNote>,
    pub direction: OwnedAccountTransactionListItemDirection,
    pub kind: OwnedAccountTransactionListItemKind,
    pub status: OwnedAccountTransactionListItemStatus,
    pub occurred_at: EventOccurredAt,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OwnedAccountTransactionListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.kind.observed_event_ids()),
        )
    }
}
