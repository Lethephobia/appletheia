use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationFragment, OrganizationInvitationFragment,
    OrganizationInvitationFragmentWriterError, UserFragment,
};
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationInvitationIssuer,
    OrganizationInvitationStatus, OrganizationRoles, UserId,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationInvitationFragmentRow {
    pub invitation_id: Uuid,
    pub organization_id: Uuid,
    pub invitee_user_id: Uuid,
    pub roles: String,
    pub issuer_type: String,
    pub issuer_user_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgOrganizationInvitationFragmentRow {
    pub fn try_into_fragment(
        self,
        organization: OrganizationFragment,
        invitee: UserFragment,
    ) -> Result<OrganizationInvitationFragment, OrganizationInvitationFragmentWriterError> {
        let row = self;
        let issuer = match (row.issuer_type.as_str(), row.issuer_user_id) {
            ("user", Some(user_id)) => OrganizationInvitationIssuer::User(
                UserId::try_from_uuid(user_id).map_err(persistence_error)?,
            ),
            ("system", None) => OrganizationInvitationIssuer::System,
            _ => {
                return Err(persistence_message(
                    "invalid organization invitation issuer",
                ));
            }
        };
        let status = match row.status.as_str() {
            "pending" => OrganizationInvitationStatus::Pending,
            "accepted" => OrganizationInvitationStatus::Accepted,
            "declined" => OrganizationInvitationStatus::Declined,
            "canceled" => OrganizationInvitationStatus::Canceled,
            "rejected" => OrganizationInvitationStatus::Rejected,
            _ => {
                return Err(persistence_message(
                    "unknown organization invitation status",
                ));
            }
        };

        Ok(OrganizationInvitationFragment {
            invitation_id: OrganizationInvitationId::try_from_uuid(row.invitation_id)
                .map_err(persistence_error)?,
            organization,
            invitee,
            roles: serde_json::from_str::<OrganizationRoles>(&row.roles)
                .map_err(persistence_error)?,
            issuer,
            expires_at: OrganizationInvitationExpiresAt::from(row.expires_at),
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
) -> OrganizationInvitationFragmentWriterError {
    OrganizationInvitationFragmentWriterError::Persistence(Box::new(error))
}

fn persistence_message(message: &'static str) -> OrganizationInvitationFragmentWriterError {
    OrganizationInvitationFragmentWriterError::Persistence(Box::new(std::io::Error::other(message)))
}
