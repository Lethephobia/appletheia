use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    OrganizationInvitationListInvitee, OrganizationInvitationListIssuer,
    OrganizationInvitationListItem, OrganizationInvitationListItemStatus,
};
use banking_iam_domain::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationRoles, UserDisplayName,
    UserId, Username,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_organization_invitation_list_item_row_error::PgOrganizationInvitationListItemRowError;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationInvitationListItemRow {
    pub invitation_id: Uuid,
    pub invitee_user_id: Uuid,
    pub roles: String,
    pub issuer_type: String,
    pub issuer_user_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub invitee_username: Option<String>,
    pub invitee_display_name: Option<String>,
    pub invitee_picture_type: Option<String>,
    pub invitee_picture_object_name: Option<String>,
    pub invitee_picture_external_url: Option<String>,
    pub invitee_source_event_id: Uuid,
    pub invitee_updated_event_id: Uuid,
}

impl PgOrganizationInvitationListItemRow {
    fn roles(value: String) -> Result<OrganizationRoles, PgOrganizationInvitationListItemRowError> {
        serde_json::from_str(&value)
            .map_err(|error| PgOrganizationInvitationListItemRowError::Roles(Box::new(error)))
    }

    fn issuer(
        issuer_type: String,
        issuer_user_id: Option<Uuid>,
    ) -> Result<OrganizationInvitationListIssuer, PgOrganizationInvitationListItemRowError> {
        match (issuer_type.as_str(), issuer_user_id) {
            ("user", Some(user_id)) => Ok(OrganizationInvitationListIssuer::User(
                UserId::try_from_uuid(user_id).map_err(|error| {
                    PgOrganizationInvitationListItemRowError::IssuerUserId(Box::new(error))
                })?,
            )),
            ("system", None) => Ok(OrganizationInvitationListIssuer::System),
            _ => Err(PgOrganizationInvitationListItemRowError::Issuer),
        }
    }

    fn status(
        value: String,
    ) -> Result<OrganizationInvitationListItemStatus, PgOrganizationInvitationListItemRowError>
    {
        match value.as_str() {
            "pending" => Ok(OrganizationInvitationListItemStatus::Pending),
            "accepted" => Ok(OrganizationInvitationListItemStatus::Accepted),
            "declined" => Ok(OrganizationInvitationListItemStatus::Declined),
            "canceled" => Ok(OrganizationInvitationListItemStatus::Canceled),
            "rejected" => Ok(OrganizationInvitationListItemStatus::Rejected),
            _ => Err(PgOrganizationInvitationListItemRowError::UnknownStatus(
                value,
            )),
        }
    }
}

impl TryFrom<PgOrganizationInvitationListItemRow> for OrganizationInvitationListItem {
    type Error = PgOrganizationInvitationListItemRowError;

    fn try_from(row: PgOrganizationInvitationListItemRow) -> Result<Self, Self::Error> {
        let invitee_username = row
            .invitee_username
            .map(Username::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationInvitationListItemRowError::InviteeUsername(Box::new(error))
            })?;
        let invitee_display_name = row
            .invitee_display_name
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationInvitationListItemRowError::InviteeDisplayName(Box::new(error))
            })?;

        Ok(Self {
            invitation_id: OrganizationInvitationId::try_from_uuid(row.invitation_id).map_err(
                |error| PgOrganizationInvitationListItemRowError::InvitationId(Box::new(error)),
            )?,
            invitee: OrganizationInvitationListInvitee {
                user_id: UserId::try_from_uuid(row.invitee_user_id).map_err(|error| {
                    PgOrganizationInvitationListItemRowError::InviteeUserId(Box::new(error))
                })?,
                username: invitee_username,
                display_name: invitee_display_name,
                picture: PgUserPictureRefColumns {
                    picture_type: row.invitee_picture_type,
                    object_name: row.invitee_picture_object_name,
                    external_url: row.invitee_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgOrganizationInvitationListItemRowError::InviteePicture(Box::new(error))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(row.invitee_source_event_id).map_err(|error| {
                        PgOrganizationInvitationListItemRowError::InviteeSourceEventId(Box::new(
                            error,
                        ))
                    })?,
                    EventId::try_from(row.invitee_updated_event_id).map_err(|error| {
                        PgOrganizationInvitationListItemRowError::InviteeUpdatedEventId(Box::new(
                            error,
                        ))
                    })?,
                ),
            },
            roles: PgOrganizationInvitationListItemRow::roles(row.roles)?,
            issuer: PgOrganizationInvitationListItemRow::issuer(
                row.issuer_type,
                row.issuer_user_id,
            )?,
            expires_at: OrganizationInvitationExpiresAt::from(row.expires_at),
            status: PgOrganizationInvitationListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgOrganizationInvitationListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgOrganizationInvitationListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
