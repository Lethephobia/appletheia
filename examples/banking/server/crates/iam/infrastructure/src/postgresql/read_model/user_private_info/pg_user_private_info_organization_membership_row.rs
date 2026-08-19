use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use banking_iam_application::{UserPrivateInfoOrganization, UserPrivateInfoOrganizationMembership};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationRoles,
};
use uuid::Uuid;

use super::pg_user_private_info_row_error::PgUserPrivateInfoRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserPrivateInfoOrganizationMembershipRow {
    pub organization_id: Uuid,
    pub roles: String,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
    pub organization_handle: Option<String>,
    pub organization_display_name: Option<String>,
    pub organization_picture_type: Option<String>,
    pub organization_picture_object_name: Option<String>,
    pub organization_picture_external_url: Option<String>,
    pub organization_source_event_id: Option<Uuid>,
    pub organization_updated_event_id: Option<Uuid>,
}

impl PgUserPrivateInfoOrganizationMembershipRow {
    fn roles(value: String) -> Result<OrganizationRoles, PgUserPrivateInfoRowError> {
        serde_json::from_str(&value)
            .map_err(|error| PgUserPrivateInfoRowError::InvalidOrganizationRoles(Box::new(error)))
    }
}

impl TryFrom<PgUserPrivateInfoOrganizationMembershipRow> for UserPrivateInfoOrganizationMembership {
    type Error = PgUserPrivateInfoRowError;

    fn try_from(row: PgUserPrivateInfoOrganizationMembershipRow) -> Result<Self, Self::Error> {
        let organization_handle = row
            .organization_handle
            .ok_or(PgUserPrivateInfoRowError::MissingOrganization)?;
        let organization_display_name = row
            .organization_display_name
            .ok_or(PgUserPrivateInfoRowError::MissingOrganization)?;
        let organization_source_event_id = row
            .organization_source_event_id
            .ok_or(PgUserPrivateInfoRowError::MissingOrganization)?;
        let organization_updated_event_id = row
            .organization_updated_event_id
            .ok_or(PgUserPrivateInfoRowError::MissingOrganization)?;

        Ok(Self {
            organization: UserPrivateInfoOrganization {
                id: OrganizationId::try_from_uuid(row.organization_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidOrganizationId(Box::new(error))
                })?,
                handle: OrganizationHandle::try_from(organization_handle).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidOrganizationHandle(Box::new(error))
                })?,
                display_name: OrganizationDisplayName::try_from(organization_display_name)
                    .map_err(|error| {
                        PgUserPrivateInfoRowError::InvalidOrganizationDisplayName(Box::new(error))
                    })?,
                picture: PgOrganizationPictureRefColumns {
                    picture_type: row.organization_picture_type,
                    object_name: row.organization_picture_object_name,
                    external_url: row.organization_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidOrganizationPicture(Box::new(error))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(organization_source_event_id).map_err(|error| {
                        PgUserPrivateInfoRowError::InvalidSourceEventId(Box::new(error))
                    })?,
                    EventId::try_from(organization_updated_event_id).map_err(|error| {
                        PgUserPrivateInfoRowError::InvalidUpdatedEventId(Box::new(error))
                    })?,
                ),
            },
            roles: PgUserPrivateInfoOrganizationMembershipRow::roles(row.roles)?,
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidSourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserPrivateInfoRowError::InvalidUpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
