use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationJoinRequestProjectionStore, OrganizationJoinRequestProjectionStoreError,
    OrganizationJoinRequestProjectionUpsert,
};
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

/// PostgreSQL-backed organization join request projection store.
pub struct PgOrganizationJoinRequestProjectionStore;

impl PgOrganizationJoinRequestProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationJoinRequestStatus) -> &'static str {
        match status {
            OrganizationJoinRequestStatus::Pending => "pending",
            OrganizationJoinRequestStatus::Approved => "approved",
            OrganizationJoinRequestStatus::Rejected => "rejected",
            OrganizationJoinRequestStatus::Canceled => "canceled",
        }
    }
}

impl Default for PgOrganizationJoinRequestProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationJoinRequestProjectionStore for PgOrganizationJoinRequestProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationJoinRequestProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationJoinRequestProjectionStoreError> {
        sqlx::query(
            r#"
            INSERT INTO organization_join_requests (
                id,
                organization_id,
                requester_id,
                status,
                created_at, updated_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                requester_id = EXCLUDED.requester_id,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_join_requests.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.organization_id.value())
        .bind(input.requester_id.value())
        .bind(Self::status_name(input.status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationJoinRequestProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationJoinRequestProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organization_join_requests
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationJoinRequestProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
