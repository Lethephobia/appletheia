use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{
    UserOrganizationMembershipListItem, UserOrganizationMembershipListOrganization,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationMembershipId,
    OrganizationRoles,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_user_organization_membership_list_item_row_error::PgUserOrganizationMembershipListItemRowError;
use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgUserOrganizationMembershipListItemRow {
    pub organization_membership_id: Uuid,
    pub organization_id: Uuid,
    pub roles: String,
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

impl TryFrom<PgUserOrganizationMembershipListItemRow> for UserOrganizationMembershipListItem {
    type Error = PgUserOrganizationMembershipListItemRowError;

    fn try_from(row: PgUserOrganizationMembershipListItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            organization_membership_id: OrganizationMembershipId::try_from_uuid(
                row.organization_membership_id,
            )
            .map_err(|error| {
                PgUserOrganizationMembershipListItemRowError::OrganizationMembershipId(Box::new(
                    error,
                ))
            })?,
            organization: UserOrganizationMembershipListOrganization {
                organization_id: OrganizationId::try_from_uuid(row.organization_id).map_err(
                    |error| {
                        PgUserOrganizationMembershipListItemRowError::OrganizationId(Box::new(
                            error,
                        ))
                    },
                )?,
                handle: OrganizationHandle::try_from(row.organization_handle).map_err(|error| {
                    PgUserOrganizationMembershipListItemRowError::OrganizationHandle(Box::new(
                        error,
                    ))
                })?,
                display_name: OrganizationDisplayName::try_from(row.organization_display_name)
                    .map_err(|error| {
                        PgUserOrganizationMembershipListItemRowError::OrganizationDisplayName(
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
                    PgUserOrganizationMembershipListItemRowError::OrganizationPicture(Box::new(
                        error,
                    ))
                })?,
                observation: ReadModelObservation::new(
                    EventId::try_from(row.organization_source_event_id).map_err(|error| {
                        PgUserOrganizationMembershipListItemRowError::OrganizationSourceEventId(
                            Box::new(error),
                        )
                    })?,
                    EventId::try_from(row.organization_updated_event_id).map_err(|error| {
                        PgUserOrganizationMembershipListItemRowError::OrganizationUpdatedEventId(
                            Box::new(error),
                        )
                    })?,
                ),
            },
            roles: serde_json::from_str::<OrganizationRoles>(&row.roles).map_err(|error| {
                PgUserOrganizationMembershipListItemRowError::Roles(Box::new(error))
            })?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgUserOrganizationMembershipListItemRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgUserOrganizationMembershipListItemRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
