use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;

use super::{OrganizationManagementInfo, OrganizationManagementInfoReaderError};

/// Loads organization-management information read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationManagementInfoReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
    ) -> Result<Option<OrganizationManagementInfo>, OrganizationManagementInfoReaderError>;
}
