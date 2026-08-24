use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationJoinRequestFragment, OrganizationJoinRequestFragmentUpsert,
    OrganizationJoinRequestFragmentWriter, OrganizationJoinRequestFragmentWriterError,
};
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};

mod pg_organization_join_request_fragment_row;
mod pg_organization_join_request_fragment_row_error;

use pg_organization_join_request_fragment_row::PgOrganizationJoinRequestFragmentRow;
use pg_organization_join_request_fragment_row_error::PgOrganizationJoinRequestFragmentRowError;

/// PostgreSQL-backed organization join request fragment writer.
pub struct PgOrganizationJoinRequestFragmentWriter;

impl PgOrganizationJoinRequestFragmentWriter {
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

    fn map_row(
        row: Option<PgOrganizationJoinRequestFragmentRow>,
    ) -> Result<Option<OrganizationJoinRequestFragment>, OrganizationJoinRequestFragmentWriterError>
    {
        let Some(join_request_row) = row else {
            return Ok(None);
        };
        OrganizationJoinRequestFragment::try_from(join_request_row).map(Some)
    }
}

impl Default for PgOrganizationJoinRequestFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationJoinRequestFragmentWriter for PgOrganizationJoinRequestFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationJoinRequestFragmentUpsert,
    ) -> Result<Option<OrganizationJoinRequestFragment>, OrganizationJoinRequestFragmentWriterError>
    {
        let row = sqlx::query_as::<_, PgOrganizationJoinRequestFragmentRow>(
            r#"
            INSERT INTO organization_join_request_fragments (
                id, organization_id, requester_user_id, status, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6, $6, $7, $7)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                requester_user_id = EXCLUDED.requester_user_id,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_join_request_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING id AS join_request_id, organization_id, requester_user_id,
                      status, created_at, source_event_id, updated_event_id
            "#,
        )
        .bind(upsert.join_request_id.value())
        .bind(upsert.organization_id.value())
        .bind(upsert.requester_user_id.value())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            OrganizationJoinRequestFragmentWriterError::Persistence(Box::new(error))
        })?;

        Self::map_row(row)
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        join_request_id: OrganizationJoinRequestId,
        status: OrganizationJoinRequestStatus,
    ) -> Result<Option<OrganizationJoinRequestFragment>, OrganizationJoinRequestFragmentWriterError>
    {
        let row = sqlx::query_as::<_, PgOrganizationJoinRequestFragmentRow>(
            r#"
            UPDATE organization_join_request_fragments
               SET status = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING id AS join_request_id, organization_id, requester_user_id,
                      status, created_at, source_event_id, updated_event_id
            "#,
        )
        .bind(join_request_id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            OrganizationJoinRequestFragmentWriterError::Persistence(Box::new(error))
        })?;

        Self::map_row(row)
    }
}
