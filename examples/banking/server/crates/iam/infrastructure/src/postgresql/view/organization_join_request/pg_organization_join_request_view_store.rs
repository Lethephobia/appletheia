use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationJoinRequestViewStore, OrganizationJoinRequestViewStoreError,
    OrganizationJoinRequestViewUpsert,
};
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

/// PostgreSQL-backed organization join request view store.
pub struct PgOrganizationJoinRequestViewStore;

impl PgOrganizationJoinRequestViewStore {
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

impl Default for PgOrganizationJoinRequestViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationJoinRequestViewStore for PgOrganizationJoinRequestViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationJoinRequestViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationJoinRequestViewStoreError> {
        sqlx::query(
            r#"
            INSERT INTO organization_join_requests (
                id,
                organization_id,
                requester_id,
                status,
                updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                requester_id = EXCLUDED.requester_id,
                status = EXCLUDED.status,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_join_requests.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.organization_id.value())
        .bind(input.requester_id.value())
        .bind(Self::status_name(input.status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationJoinRequestViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationJoinRequestViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organization_join_requests
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
        .map_err(|e| OrganizationJoinRequestViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
