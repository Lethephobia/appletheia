use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_application::{OrganizationFragment, UserFragment};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationWebsiteUrl, UserId,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;

use super::pg_organization_fragment_row_error::PgOrganizationFragmentRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOrganizationFragmentRow {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub owner_since: DateTime<Utc>,
    pub owner_source_event_id: Uuid,
    pub owner_updated_event_id: Uuid,
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
}

impl PgOrganizationFragmentRow {
    pub fn try_into_fragment(
        self,
        owner: UserFragment,
    ) -> Result<OrganizationFragment, PgOrganizationFragmentRowError> {
        let row = self;
        let description = row
            .description
            .map(OrganizationDescription::try_from)
            .transpose()
            .map_err(|error| PgOrganizationFragmentRowError::Description(Box::new(error)))?;
        let website_url = row
            .website_url
            .map(OrganizationWebsiteUrl::try_from)
            .transpose()
            .map_err(|error| PgOrganizationFragmentRowError::WebsiteUrl(Box::new(error)))?;

        let owner_user_id = UserId::try_from_uuid(row.owner_user_id)
            .map_err(|error| PgOrganizationFragmentRowError::OwnerUserId(Box::new(error)))?;
        if owner.id != owner_user_id {
            return Err(PgOrganizationFragmentRowError::OwnerMismatch {
                expected: owner_user_id,
                actual: owner.id,
            });
        }

        Ok(OrganizationFragment {
            id: OrganizationId::try_from_uuid(row.id)
                .map_err(|error| PgOrganizationFragmentRowError::OrganizationId(Box::new(error)))?,
            owner,
            owner_since: EventOccurredAt::from(row.owner_since),
            owner_observation: ReadModelObservation::new(
                EventId::try_from(row.owner_source_event_id).map_err(|error| {
                    PgOrganizationFragmentRowError::OwnerSourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.owner_updated_event_id).map_err(|error| {
                    PgOrganizationFragmentRowError::OwnerUpdatedEventId(Box::new(error))
                })?,
            ),
            handle: OrganizationHandle::try_from(row.handle)
                .map_err(|error| PgOrganizationFragmentRowError::Handle(Box::new(error)))?,
            display_name: OrganizationDisplayName::try_from(row.display_name)
                .map_err(|error| PgOrganizationFragmentRowError::DisplayName(Box::new(error)))?,
            description,
            website_url,
            picture: PgOrganizationPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(|error| PgOrganizationFragmentRowError::Picture(Box::new(error)))?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(|error| {
                    PgOrganizationFragmentRowError::SourceEventId(Box::new(error))
                })?,
                EventId::try_from(row.updated_event_id).map_err(|error| {
                    PgOrganizationFragmentRowError::UpdatedEventId(Box::new(error))
                })?,
            ),
        })
    }
}
