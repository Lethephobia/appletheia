use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationFragment, OrganizationMembershipFragment,
    OrganizationMembershipFragmentWriterError, UserFragment,
};
use banking_iam_domain::OrganizationRoles;
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

impl PgOrganizationMembershipFragmentRow {
    pub fn try_into_fragment(
        self,
        user: UserFragment,
        organization: OrganizationFragment,
    ) -> Result<OrganizationMembershipFragment, OrganizationMembershipFragmentWriterError> {
        let row = self;
        Ok(OrganizationMembershipFragment {
            user,
            organization,
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
