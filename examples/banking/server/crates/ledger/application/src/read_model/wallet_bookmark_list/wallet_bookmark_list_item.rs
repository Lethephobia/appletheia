use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};

/// Read model for one wallet bookmark list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WalletBookmarkListItem {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl WalletBookmarkListItem {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}
