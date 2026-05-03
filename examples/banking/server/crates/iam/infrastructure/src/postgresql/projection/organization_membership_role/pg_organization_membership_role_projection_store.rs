use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationMembershipRoleProjectionStore, OrganizationMembershipRoleProjectionStoreError,
    OrganizationMembershipRoleProjectionUpsert,
};
use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

/// PostgreSQL-backed membership role projection store.
pub struct PgOrganizationMembershipRoleProjectionStore;

impl PgOrganizationMembershipRoleProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn role_name(role: OrganizationRole) -> &'static str {
        match role {
            OrganizationRole::Admin => "admin",
            OrganizationRole::FinanceManager => "finance_manager",
            OrganizationRole::Treasurer => "treasurer",
        }
    }
}

impl Default for PgOrganizationMembershipRoleProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationMembershipRoleProjectionStore for PgOrganizationMembershipRoleProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipRoleProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError> {
        sqlx::query(
            r#"
            INSERT INTO organization_membership_roles (
                organization_membership_id,
                role,
                updated_event_sequence
            )
            SELECT $1, $2, $3
             WHERE EXISTS (
                SELECT 1
                  FROM organization_memberships
                 WHERE id = $1
                   AND updated_event_sequence < $3
             )
            ON CONFLICT (organization_membership_id, role) DO UPDATE SET
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_membership_roles.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.organization_membership_id.value())
        .bind(Self::role_name(input.role))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipRoleProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        role: OrganizationRole,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM organization_membership_roles
             WHERE organization_membership_id = $1
               AND role = $2
               AND updated_event_sequence < $3
            "#,
        )
        .bind(organization_membership_id.value())
        .bind(Self::role_name(role))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipRoleProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_by_membership(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipRoleProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM organization_membership_roles
             WHERE organization_membership_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(organization_membership_id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipRoleProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
