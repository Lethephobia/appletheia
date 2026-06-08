use appletheia::domain::{AggregateId, EventId};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{
    OwnedAccountListOwner, OwnedAccountListOwnerOrganization, OwnedAccountListOwnerUser,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_owned_account_list_owner_row_error::PgOwnedAccountListOwnerRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountListOwnerRow {
    pub owner_type: String,
    pub owner_id: uuid::Uuid,
    pub owner_user_username: Option<String>,
    pub owner_user_display_name: Option<String>,
    pub owner_user_picture_type: Option<String>,
    pub owner_user_picture_object_name: Option<String>,
    pub owner_user_picture_external_url: Option<String>,
    pub owner_organization_handle: Option<String>,
    pub owner_organization_display_name: Option<String>,
    pub owner_organization_picture_type: Option<String>,
    pub owner_organization_picture_object_name: Option<String>,
    pub owner_organization_picture_external_url: Option<String>,
    pub source_event_id: Option<uuid::Uuid>,
    pub updated_event_id: Option<uuid::Uuid>,
}

impl PgOwnedAccountListOwnerRow {
    fn observation(&self) -> Result<ReadModelObservation, PgOwnedAccountListOwnerRowError> {
        let source_event_id =
            self.source_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgOwnedAccountListOwnerRowError::MissingUserOwner,
                    "organization" => PgOwnedAccountListOwnerRowError::MissingOrganizationOwner,
                    _ => PgOwnedAccountListOwnerRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        let updated_event_id =
            self.updated_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgOwnedAccountListOwnerRowError::MissingUserOwner,
                    "organization" => PgOwnedAccountListOwnerRowError::MissingOrganizationOwner,
                    _ => PgOwnedAccountListOwnerRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgOwnedAccountListOwnerRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgOwnedAccountListOwnerRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgOwnedAccountListOwnerRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgOwnedAccountListOwnerRowError::InvalidUsername(Box::new(error)))
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgOwnedAccountListOwnerRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOwnedAccountListOwnerRowError::InvalidUserDisplayName(Box::new(error))
            })
    }
}

impl TryFrom<PgOwnedAccountListOwnerRow> for OwnedAccountListOwner {
    type Error = PgOwnedAccountListOwnerRowError;

    fn try_from(row: PgOwnedAccountListOwnerRow) -> Result<Self, Self::Error> {
        let observation = row.observation()?;

        match row.owner_type.as_str() {
            "user" => Ok(Self::User(OwnedAccountListOwnerUser {
                id: UserId::try_from_uuid(row.owner_id).map_err(|error| {
                    PgOwnedAccountListOwnerRowError::InvalidUserOwnerId(Box::new(error))
                })?,
                username: PgOwnedAccountListOwnerRow::optional_username(row.owner_user_username)?,
                display_name: PgOwnedAccountListOwnerRow::optional_user_display_name(
                    row.owner_user_display_name,
                )?,
                picture: PgUserPictureRefColumns {
                    picture_type: row.owner_user_picture_type,
                    object_name: row.owner_user_picture_object_name,
                    external_url: row.owner_user_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgOwnedAccountListOwnerRowError::InvalidUserPicture(Box::new(error))
                })?,
                observation,
            })),
            "organization" => {
                let handle = row
                    .owner_organization_handle
                    .ok_or(PgOwnedAccountListOwnerRowError::MissingOrganizationOwner)?;
                let display_name = row
                    .owner_organization_display_name
                    .ok_or(PgOwnedAccountListOwnerRowError::MissingOrganizationOwner)?;

                Ok(Self::Organization(OwnedAccountListOwnerOrganization {
                    id: OrganizationId::try_from_uuid(row.owner_id).map_err(|error| {
                        PgOwnedAccountListOwnerRowError::InvalidOrganizationOwnerId(Box::new(error))
                    })?,
                    handle: OrganizationHandle::try_from(handle).map_err(|error| {
                        PgOwnedAccountListOwnerRowError::InvalidOrganizationHandle(Box::new(error))
                    })?,
                    display_name: OrganizationDisplayName::try_from(display_name).map_err(
                        |error| {
                            PgOwnedAccountListOwnerRowError::InvalidOrganizationDisplayName(
                                Box::new(error),
                            )
                        },
                    )?,
                    picture: PgOrganizationPictureRefColumns {
                        picture_type: row.owner_organization_picture_type,
                        object_name: row.owner_organization_picture_object_name,
                        external_url: row.owner_organization_picture_external_url,
                    }
                    .into_picture()
                    .map_err(|error| {
                        PgOwnedAccountListOwnerRowError::InvalidOrganizationPicture(Box::new(error))
                    })?,
                    observation,
                }))
            }
            value => Err(PgOwnedAccountListOwnerRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }
}
