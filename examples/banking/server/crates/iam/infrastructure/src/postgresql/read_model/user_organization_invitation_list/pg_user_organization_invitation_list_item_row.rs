use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    InternalOrganizationSummaryPart, UserOrganizationInvitationListItemPart,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationInvitationExpiresAt,
    OrganizationInvitationId, OrganizationRoles, UserId,
};
use banking_iam_domain::{OrganizationInvitationIssuer, OrganizationInvitationStatus};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_user_organization_invitation_list_item_row_error::PgUserOrganizationInvitationListItemRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserOrganizationInvitationListItemRow {
    pub invitation_id: Uuid,
    pub organization_id: Uuid,
    pub roles: String,
    pub issuer_type: String,
    pub issuer_user_id: Option<Uuid>,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub organization_handle: String,
    pub organization_display_name: String,
    pub organization_picture_type: Option<String>,
    pub organization_picture_object_name: Option<String>,
    pub organization_picture_external_url: Option<String>,
    pub organization_source_event_id: Uuid,
    pub organization_updated_event_id: Uuid,
}

impl PgUserOrganizationInvitationListItemRow {
    fn roles(
        value: String,
    ) -> Result<OrganizationRoles, PgUserOrganizationInvitationListItemRowError> {
        serde_json::from_str(&value)
            .map_err(|error| PgUserOrganizationInvitationListItemRowError::Roles(Box::new(error)))
    }

    fn issuer(
        issuer_type: String,
        issuer_user_id: Option<Uuid>,
    ) -> Result<OrganizationInvitationIssuer, PgUserOrganizationInvitationListItemRowError> {
        match (issuer_type.as_str(), issuer_user_id) {
            ("user", Some(user_id)) => Ok(OrganizationInvitationIssuer::User(
                UserId::try_from_uuid(user_id).map_err(|error| {
                    PgUserOrganizationInvitationListItemRowError::IssuerUserId(Box::new(error))
                })?,
            )),
            ("system", None) => Ok(OrganizationInvitationIssuer::System),
            _ => Err(PgUserOrganizationInvitationListItemRowError::Issuer),
        }
    }

    fn status(
        value: String,
    ) -> Result<OrganizationInvitationStatus, PgUserOrganizationInvitationListItemRowError> {
        match value.as_str() {
            "pending" => Ok(OrganizationInvitationStatus::Pending),
            "accepted" => Ok(OrganizationInvitationStatus::Accepted),
            "declined" => Ok(OrganizationInvitationStatus::Declined),
            "canceled" => Ok(OrganizationInvitationStatus::Canceled),
            "rejected" => Ok(OrganizationInvitationStatus::Rejected),
            _ => Err(PgUserOrganizationInvitationListItemRowError::UnknownStatus(
                value,
            )),
        }
    }
}

impl TryFrom<PgUserOrganizationInvitationListItemRow> for UserOrganizationInvitationListItemPart {
    type Error = PgUserOrganizationInvitationListItemRowError;

    fn try_from(row: PgUserOrganizationInvitationListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            invitation_id: OrganizationInvitationId::try_from_uuid(row.invitation_id).map_err(
                |error| PgUserOrganizationInvitationListItemRowError::InvitationId(Box::new(error)),
            )?,
            organization: InternalOrganizationSummaryPart {
                organization_id: OrganizationId::try_from_uuid(row.organization_id).map_err(
                    |error| {
                        PgUserOrganizationInvitationListItemRowError::OrganizationId(Box::new(
                            error,
                        ))
                    },
                )?,
                handle: OrganizationHandle::try_from(row.organization_handle).map_err(|error| {
                    PgUserOrganizationInvitationListItemRowError::OrganizationHandle(Box::new(
                        error,
                    ))
                })?,
                display_name: OrganizationDisplayName::try_from(row.organization_display_name)
                    .map_err(|error| {
                        PgUserOrganizationInvitationListItemRowError::OrganizationDisplayName(
                            Box::new(error),
                        )
                    })?,
                picture: PgOrganizationPictureRefColumns {
                    picture_type: row.organization_picture_type,
                    object_name: row.organization_picture_object_name,
                    external_url: row.organization_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgUserOrganizationInvitationListItemRowError::OrganizationPicture(Box::new(
                        error,
                    ))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(row.organization_source_event_id).map_err(|error| {
                        PgUserOrganizationInvitationListItemRowError::OrganizationSourceEventId(
                            Box::new(error),
                        )
                    })?,
                    EventId::try_from(row.organization_updated_event_id).map_err(|error| {
                        PgUserOrganizationInvitationListItemRowError::OrganizationUpdatedEventId(
                            Box::new(error),
                        )
                    })?,
                ),
            },
            roles: PgUserOrganizationInvitationListItemRow::roles(row.roles)?,
            issuer: PgUserOrganizationInvitationListItemRow::issuer(
                row.issuer_type,
                row.issuer_user_id,
            )?,
            expires_at: OrganizationInvitationExpiresAt::from(row.expires_at),
            status: PgUserOrganizationInvitationListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserOrganizationInvitationListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserOrganizationInvitationListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
