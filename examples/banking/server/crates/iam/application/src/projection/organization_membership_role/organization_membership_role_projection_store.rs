use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

use super::{
    OrganizationMembershipRoleProjectionStoreError, OrganizationMembershipRoleProjectionUpsert,
};

/// Persists normalized organization membership role projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipRoleProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipRoleProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        role: OrganizationRole,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError>;

    async fn delete_by_membership(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError>;
}
