use appletheia::application::unit_of_work::UnitOfWork;

use crate::pagination::{CursorOptions, Page, PageSize};

use super::{
    PublicAccountListItem, PublicAccountListItemCriteria, PublicAccountListItemCursor,
    PublicAccountListItemReaderError, PublicAccountListItemSortKey,
};

/// Loads public account list read models from normalized query-side tables.
#[allow(async_fn_in_trait)]
pub trait PublicAccountListItemReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicAccountListItemCriteria,
        cursor_options: Option<
            CursorOptions<PublicAccountListItemSortKey, PublicAccountListItemCursor>,
        >,
        limit: PageSize,
    ) -> Result<
        Page<PublicAccountListItem, PublicAccountListItemCursor>,
        PublicAccountListItemReaderError,
    >;
}
