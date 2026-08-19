use appletheia::application::read_model::pagination::{CursorWindow, Sort};
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;

use super::{
    OrganizationInvitationList, OrganizationInvitationListCriteria,
    OrganizationInvitationListCursor, OrganizationInvitationListReaderError,
    OrganizationInvitationListSortKey,
};

/// Loads organization invitation lists.
#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationInvitationListCriteria,
        sort: Sort<OrganizationInvitationListSortKey>,
        page: CursorWindow<OrganizationInvitationListCursor>,
    ) -> Result<OrganizationInvitationList, OrganizationInvitationListReaderError>;
}
