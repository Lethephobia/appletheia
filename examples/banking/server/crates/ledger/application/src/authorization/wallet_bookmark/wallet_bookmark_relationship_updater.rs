use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::wallet_bookmark::{WalletBookmarkId, WalletBookmarkOwner};

use super::WalletBookmarkRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait WalletBookmarkRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        wallet_bookmark_id: WalletBookmarkId,
        owner: WalletBookmarkOwner,
    ) -> Result<(), WalletBookmarkRelationshipUpdaterError>;
}
