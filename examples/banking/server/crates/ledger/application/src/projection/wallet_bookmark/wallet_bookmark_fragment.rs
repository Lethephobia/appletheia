use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId,
};
use serde::{Deserialize, Serialize};

use super::FragmentOwner;

/// Complete wallet bookmark fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletBookmarkFragment {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub owner: FragmentOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for WalletBookmarkFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.owner
            .observations()
            .into_iter()
            .chain([self.observation])
            .collect()
    }
}

impl ReadModelFragment for WalletBookmarkFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("wallet_bookmark_fragment");

    type Key = WalletBookmarkId;

    fn key(&self) -> Self::Key {
        self.wallet_bookmark_id
    }
}
