use appletheia::application::unit_of_work::UnitOfWork;

use crate::query::Page;

use super::{
    OwnedAccountListCursor, OwnedAccountListItem, OwnedAccountListQuery, OwnedAccountListStoreError,
};

/// Loads account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait OwnedAccountListStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        query: &OwnedAccountListQuery,
    ) -> Result<Page<OwnedAccountListItem, OwnedAccountListCursor>, OwnedAccountListStoreError>;
}
