use serde::{Deserialize, Serialize};

use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};

use super::WalletBookmarkFragment;

/// Read model for one wallet bookmark list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletBookmarkListItemPart {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl From<WalletBookmarkFragment> for WalletBookmarkListItemPart {
    fn from(fragment: WalletBookmarkFragment) -> Self {
        Self {
            wallet_bookmark_id: fragment.wallet_bookmark_id,
            owner: fragment.owner.wallet_bookmark_owner(),
            display_name: fragment.display_name,
            description: fragment.description,
            token_account_owner_address: fragment.token_account_owner_address,
            created_at: fragment.created_at,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for WalletBookmarkListItemPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}
