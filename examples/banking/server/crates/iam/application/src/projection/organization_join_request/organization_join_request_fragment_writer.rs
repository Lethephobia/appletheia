use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

use super::{
    OrganizationJoinRequestFragment, OrganizationJoinRequestFragmentUpsert,
    OrganizationJoinRequestFragmentWriterError,
};

/// Persists organization join request fragments independently of composed read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationJoinRequestFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationJoinRequestFragmentUpsert,
    ) -> Result<Option<OrganizationJoinRequestFragment>, OrganizationJoinRequestFragmentWriterError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        join_request_id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
    ) -> Result<Option<OrganizationJoinRequestFragment>, OrganizationJoinRequestFragmentWriterError>;
}
