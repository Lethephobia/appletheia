use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    InternalOrganizationDetailsPart, InternalUserSummaryPart, OrganizationManagementInfo,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationWebsiteUrl, UserDisplayName, UserId, Username,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_organization_management_info_row_error::PgOrganizationManagementInfoRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use crate::postgresql::pg_user_picture_ref_columns::PgUserPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationManagementInfoRow {
    pub organization_id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub picture_type: Option<String>,
    pub picture_object_name: Option<String>,
    pub picture_external_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub owner_user_id: Uuid,
    pub owner_since: DateTime<Utc>,
    pub ownership_source_event_id: Uuid,
    pub ownership_updated_event_id: Uuid,
    pub owner_username: Option<String>,
    pub owner_display_name: Option<String>,
    pub owner_picture_type: Option<String>,
    pub owner_picture_object_name: Option<String>,
    pub owner_picture_external_url: Option<String>,
    pub owner_source_event_id: Uuid,
    pub owner_updated_event_id: Uuid,
}

impl TryFrom<PgOrganizationManagementInfoRow> for OrganizationManagementInfo {
    type Error = PgOrganizationManagementInfoRowError;

    fn try_from(row: PgOrganizationManagementInfoRow) -> Result<Self, Self::Error> {
        let description = row
            .description
            .map(OrganizationDescription::try_from)
            .transpose()
            .map_err(|error| PgOrganizationManagementInfoRowError::Description(Box::new(error)))?;
        let website_url = row
            .website_url
            .map(OrganizationWebsiteUrl::try_from)
            .transpose()
            .map_err(|error| PgOrganizationManagementInfoRowError::WebsiteUrl(Box::new(error)))?;
        let owner_username = row
            .owner_username
            .map(Username::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationManagementInfoRowError::OwnerUsername(Box::new(error))
            })?;
        let owner_display_name = row
            .owner_display_name
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOrganizationManagementInfoRowError::OwnerDisplayName(Box::new(error))
            })?;

        let organization_id =
            OrganizationId::try_from_uuid(row.organization_id).map_err(|error| {
                PgOrganizationManagementInfoRowError::OrganizationId(Box::new(error))
            })?;
        let owner_user_id = UserId::try_from_uuid(row.owner_user_id)
            .map_err(|error| PgOrganizationManagementInfoRowError::OwnerUserId(Box::new(error)))?;
        let organization_observation = ReadModelObservation::new(
            EventId::try_from(row.source_event_id).map_err(|error| {
                PgOrganizationManagementInfoRowError::SourceEventId(Box::new(error))
            })?,
            EventId::try_from(row.updated_event_id).map_err(|error| {
                PgOrganizationManagementInfoRowError::UpdatedEventId(Box::new(error))
            })?,
        );
        let owner_observation = ReadModelObservation::new(
            EventId::try_from(row.owner_source_event_id).map_err(|error| {
                PgOrganizationManagementInfoRowError::OwnerSourceEventId(Box::new(error))
            })?,
            EventId::try_from(row.owner_updated_event_id).map_err(|error| {
                PgOrganizationManagementInfoRowError::OwnerUpdatedEventId(Box::new(error))
            })?,
        );

        Ok(Self {
            organization: InternalOrganizationDetailsPart {
                organization_id,
                handle: OrganizationHandle::try_from(row.handle).map_err(|error| {
                    PgOrganizationManagementInfoRowError::Handle(Box::new(error))
                })?,
                display_name: OrganizationDisplayName::try_from(row.display_name).map_err(
                    |error| PgOrganizationManagementInfoRowError::DisplayName(Box::new(error)),
                )?,
                picture: PgOrganizationPictureRefColumns {
                    picture_type: row.picture_type,
                    object_name: row.picture_object_name,
                    external_url: row.picture_external_url,
                }
                .into_picture()
                .map_err(|error| PgOrganizationManagementInfoRowError::Picture(Box::new(error)))?,
                observation: organization_observation,
                description,
                website_url,
                created_at: EventOccurredAt::from(row.created_at),
            },
            owner: InternalUserSummaryPart {
                user_id: owner_user_id,
                username: owner_username,
                display_name: owner_display_name,
                picture: PgUserPictureRefColumns {
                    picture_type: row.owner_picture_type,
                    object_name: row.owner_picture_object_name,
                    external_url: row.owner_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgOrganizationManagementInfoRowError::OwnerPicture(Box::new(error))
                })?,
                observation: owner_observation,
            },
            owner_since: EventOccurredAt::from(row.owner_since),
            owner_observation: ReadModelObservation::new(
                EventId::try_from(row.ownership_source_event_id).map_err(|error| {
                    PgOrganizationManagementInfoRowError::OwnerSourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.ownership_updated_event_id).map_err(|error| {
                    PgOrganizationManagementInfoRowError::OwnerUpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
