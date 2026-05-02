use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

use super::{OrganizationMembershipRoleViewStoreError, OrganizationMembershipRoleViewUpsert};

/// Persists normalized organization membership role views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipRoleViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipRoleViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleViewStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        role: OrganizationRole,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleViewStoreError>;

    async fn delete_by_membership(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleViewStoreError>;
}
