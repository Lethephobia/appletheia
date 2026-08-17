use appletheia::application::read_model::pagination::{CursorPage, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;

use super::{
    OrganizationMemberList, OrganizationMemberListCriteria, OrganizationMemberListCursor,
    OrganizationMemberListReaderError, OrganizationMemberListSortKey,
};

/// Loads organization member list read models from query-side tables.
#[allow(async_fn_in_trait)]
pub trait OrganizationMemberListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationMemberListCriteria,
        sort: Sort<OrganizationMemberListSortKey>,
        page: CursorPage<OrganizationMemberListCursor>,
    ) -> Result<OrganizationMemberList, OrganizationMemberListReaderError>;
}
