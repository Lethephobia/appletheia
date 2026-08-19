use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};
use serde::{Deserialize, Serialize};

/// Normalized wallet bookmark fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WalletBookmarkFragment {
    pub wallet_bookmark_id: WalletBookmarkId,
    pub owner: WalletBookmarkOwner,
    pub display_name: Option<WalletBookmarkDisplayName>,
    pub description: Option<WalletBookmarkDescription>,
    pub token_account_owner_address: TokenAccountOwnerAddress,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for WalletBookmarkFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for WalletBookmarkFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("wallet_bookmark_fragment");

    type Key = WalletBookmarkId;

    fn key(&self) -> Self::Key {
        self.wallet_bookmark_id
    }
}
