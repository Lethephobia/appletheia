use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;

use super::{
    OrganizationJoinRequestList, OrganizationJoinRequestListCriteria,
    OrganizationJoinRequestListCursor, OrganizationJoinRequestListReaderError,
    OrganizationJoinRequestListSortKey,
};

/// Loads organization join request lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationJoinRequestListCriteria,
        sort: Sort<OrganizationJoinRequestListSortKey>,
        page: CursorWindow<OrganizationJoinRequestListCursor>,
    ) -> Result<OrganizationJoinRequestList, OrganizationJoinRequestListReaderError>;
}
