use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationJoinRequestFragment, OrganizationJoinRequestFragmentWriterError,
};
use banking_iam_domain::{
    OrganizationId, OrganizationJoinRequestId, OrganizationJoinRequestStatus, UserId,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::PgOrganizationJoinRequestFragmentRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationJoinRequestFragmentRow {
    pub join_request_id: Uuid,
    pub organization_id: Uuid,
    pub requester_user_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgOrganizationJoinRequestFragmentRow> for OrganizationJoinRequestFragment {
    type Error = OrganizationJoinRequestFragmentWriterError;

    fn try_from(row: PgOrganizationJoinRequestFragmentRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "pending" => OrganizationJoinRequestStatus::Pending,
            "approved" => OrganizationJoinRequestStatus::Approved,
            "rejected" => OrganizationJoinRequestStatus::Rejected,
            "canceled" => OrganizationJoinRequestStatus::Canceled,
            _ => {
                return Err(persistence_error(
                    PgOrganizationJoinRequestFragmentRowError::Status(row.status.clone()),
                ));
            }
        };

        Ok(OrganizationJoinRequestFragment {
            join_request_id: OrganizationJoinRequestId::try_from_uuid(row.join_request_id)
                .map_err(persistence_error)?,
            organization_id: OrganizationId::try_from_uuid(row.organization_id)
                .map_err(persistence_error)?,
            requester_user_id: UserId::try_from_uuid(row.requester_user_id)
                .map_err(persistence_error)?,
            status,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence_error)?,
                EventId::try_from(row.updated_event_id).map_err(persistence_error)?,
            ),
        })
    }
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> OrganizationJoinRequestFragmentWriterError {
    OrganizationJoinRequestFragmentWriterError::Persistence(Box::new(error))
}
