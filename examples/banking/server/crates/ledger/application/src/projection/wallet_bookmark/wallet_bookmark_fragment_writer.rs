use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId,
};

use super::{
    WalletBookmarkFragment, WalletBookmarkFragmentUpsert, WalletBookmarkFragmentWriterError,
};

#[allow(async_fn_in_trait)]
pub trait WalletBookmarkFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: WalletBookmarkFragmentUpsert,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError>;

    async fn delete_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
    ) -> Result<bool, WalletBookmarkFragmentWriterError>;
}
