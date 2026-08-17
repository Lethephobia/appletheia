use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};

use super::{
    OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    OrganizationMembershipFragmentUpsert, OrganizationMembershipFragmentWriterError,
};

/// Persists organization membership fragments independently of composed read models.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationMembershipFragmentUpsert,
    ) -> Result<Option<OrganizationMembershipFragment>, OrganizationMembershipFragmentWriterError>;

    async fn update_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
        roles: OrganizationRoles,
    ) -> Result<Option<OrganizationMembershipFragment>, OrganizationMembershipFragmentWriterError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
    ) -> Result<bool, OrganizationMembershipFragmentWriterError>;

    async fn delete_for_user(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
    ) -> Result<Vec<OrganizationMembershipFragmentKey>, OrganizationMembershipFragmentWriterError>;
}
