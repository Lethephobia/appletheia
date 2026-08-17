use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    PublicOrganizationList, PublicOrganizationListCriteria, PublicOrganizationListCursor,
    PublicOrganizationListReaderError, PublicOrganizationListSortKey,
};

/// Loads public organization list read models from query-side tables.
#[allow(async_fn_in_trait)]
pub trait PublicOrganizationListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicOrganizationListCriteria,
        sort: Sort<PublicOrganizationListSortKey>,
        page: CursorPage<PublicOrganizationListCursor>,
    ) -> Result<PublicOrganizationList, PublicOrganizationListReaderError>;
}
