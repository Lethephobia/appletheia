use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationMembershipViewStore, OrganizationMembershipViewStoreError,
    OrganizationMembershipViewUpsert,
};
use banking_iam_domain::{OrganizationMembershipId, OrganizationMembershipStatus};

/// PostgreSQL-backed membership view store.
pub struct PgOrganizationMembershipViewStore;

impl PgOrganizationMembershipViewStore {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationMembershipStatus) -> &'static str {
        match status {
            OrganizationMembershipStatus::Active => "active",
            OrganizationMembershipStatus::Inactive => "inactive",
            OrganizationMembershipStatus::Removed => "removed",
        }
    }
}

impl Default for PgOrganizationMembershipViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationMembershipViewStore for PgOrganizationMembershipViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationMembershipViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError> {
        sqlx::query(
            r#"
            INSERT INTO organization_memberships (
                id,
                organization_id,
                user_id,
                status,
                updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                user_id = EXCLUDED.user_id,
                status = EXCLUDED.status,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_memberships.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.organization_id.value())
        .bind(input.user_id.value())
        .bind(Self::status_name(input.status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        status: OrganizationMembershipStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organization_memberships
               SET status = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationMembershipId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationMembershipViewStoreError> {
        sqlx::query(
            r#"
            DELETE FROM organization_memberships
             WHERE id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationMembershipViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
