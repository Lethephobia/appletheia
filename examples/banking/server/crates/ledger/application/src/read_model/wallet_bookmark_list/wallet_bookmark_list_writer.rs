use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{WalletBookmarkListUpsert, WalletBookmarkListWriterError};

#[allow(async_fn_in_trait)]
pub trait WalletBookmarkListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: WalletBookmarkListUpsert,
    ) -> Result<(), WalletBookmarkListWriterError>;

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<(), WalletBookmarkListWriterError>;

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<(), WalletBookmarkListWriterError>;

    async fn delete_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
    ) -> Result<(), WalletBookmarkListWriterError>;
}
