use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationMembershipFragment, OrganizationMembershipFragmentWriterError,
};
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};
use sqlx::types::Json;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationMembershipFragmentRow {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub roles: Json<OrganizationRoles>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgOrganizationMembershipFragmentRow> for OrganizationMembershipFragment {
    type Error = OrganizationMembershipFragmentWriterError;

    fn try_from(row: PgOrganizationMembershipFragmentRow) -> Result<Self, Self::Error> {
        Ok(OrganizationMembershipFragment {
            user_id: UserId::try_from_uuid(row.user_id).map_err(persistence_error)?,
            organization_id: OrganizationId::try_from_uuid(row.organization_id)
                .map_err(persistence_error)?,
            roles: row.roles.0,
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
) -> OrganizationMembershipFragmentWriterError {
    OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
}
