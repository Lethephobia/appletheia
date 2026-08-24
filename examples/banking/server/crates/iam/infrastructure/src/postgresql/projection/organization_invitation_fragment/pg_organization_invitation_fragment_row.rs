use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationInvitationFragment, OrganizationInvitationFragmentWriterError,
};
use banking_iam_domain::{
    OrganizationId, OrganizationInvitationExpiresAt, OrganizationInvitationId,
    OrganizationInvitationIssuer, OrganizationInvitationStatus, OrganizationRoles, UserId,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::PgOrganizationInvitationFragmentRowError;

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

impl TryFrom<PgOrganizationInvitationFragmentRow> for OrganizationInvitationFragment {
    type Error = OrganizationInvitationFragmentWriterError;

    fn try_from(row: PgOrganizationInvitationFragmentRow) -> Result<Self, Self::Error> {
        let issuer = match (row.issuer_type.as_str(), row.issuer_user_id) {
            ("user", Some(user_id)) => OrganizationInvitationIssuer::User(
                UserId::try_from_uuid(user_id).map_err(persistence_error)?,
            ),
            ("system", None) => OrganizationInvitationIssuer::System,
            _ => {
                return Err(persistence_error(
                    PgOrganizationInvitationFragmentRowError::Issuer {
                        issuer_type: row.issuer_type.clone(),
                        user_id_present: row.issuer_user_id.is_some(),
                    },
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
                return Err(persistence_error(
                    PgOrganizationInvitationFragmentRowError::Status(row.status.clone()),
                ));
            }
        };

        Ok(OrganizationInvitationFragment {
            invitation_id: OrganizationInvitationId::try_from_uuid(row.invitation_id)
                .map_err(persistence_error)?,
            organization_id: OrganizationId::try_from_uuid(row.organization_id)
                .map_err(persistence_error)?,
            invitee_user_id: UserId::try_from_uuid(row.invitee_user_id)
                .map_err(persistence_error)?,
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
