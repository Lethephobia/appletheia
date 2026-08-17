use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationFragment, OrganizationJoinRequestFragment,
    OrganizationJoinRequestFragmentWriterError, UserFragment,
};
use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestStatus};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

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

impl PgOrganizationJoinRequestFragmentRow {
    pub fn try_into_fragment(
        self,
        organization: OrganizationFragment,
        requester: UserFragment,
    ) -> Result<OrganizationJoinRequestFragment, OrganizationJoinRequestFragmentWriterError> {
        let row = self;
        let status = match row.status.as_str() {
            "pending" => OrganizationJoinRequestStatus::Pending,
            "approved" => OrganizationJoinRequestStatus::Approved,
            "rejected" => OrganizationJoinRequestStatus::Rejected,
            "canceled" => OrganizationJoinRequestStatus::Canceled,
            _ => {
                return Err(persistence_message(
                    "unknown organization join request status",
                ));
            }
        };

        Ok(OrganizationJoinRequestFragment {
            join_request_id: OrganizationJoinRequestId::try_from_uuid(row.join_request_id)
                .map_err(persistence_error)?,
            organization,
            requester,
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

fn persistence_message(message: &'static str) -> OrganizationJoinRequestFragmentWriterError {
    OrganizationJoinRequestFragmentWriterError::Persistence(Box::new(std::io::Error::other(
        message,
    )))
}
