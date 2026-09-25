use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::OrganizationId;

use super::{OrganizationInternalInfo, OrganizationInternalInfoReaderError};

/// Loads organization-internal information read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationInternalInfoReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
    ) -> Result<Option<OrganizationInternalInfo>, OrganizationInternalInfoReaderError>;
}
