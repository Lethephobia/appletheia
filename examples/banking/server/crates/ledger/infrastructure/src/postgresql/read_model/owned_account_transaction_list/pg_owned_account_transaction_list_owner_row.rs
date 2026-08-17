use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{
    OwnedAccountTransactionListOwner, OwnedAccountTransactionListOwnerOrganizationPart,
    OwnedAccountTransactionListOwnerUserPart,
};
use uuid::Uuid;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_owned_account_transaction_list_owner_row_error::PgOwnedAccountTransactionListOwnerRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountTransactionListOwnerRow {
    pub owner_type: String,
    pub owner_id: Uuid,
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
    pub source_event_id: Option<Uuid>,
    pub updated_event_id: Option<Uuid>,
}

impl PgOwnedAccountTransactionListOwnerRow {
    fn observation(
        &self,
    ) -> Result<ReadModelObservation, PgOwnedAccountTransactionListOwnerRowError> {
        let source_event_id =
            self.source_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgOwnedAccountTransactionListOwnerRowError::MissingUserOwner,
                    "organization" => {
                        PgOwnedAccountTransactionListOwnerRowError::MissingOrganizationOwner
                    }
                    _ => PgOwnedAccountTransactionListOwnerRowError::UnknownOwnerType(
                        self.owner_type.clone(),
                    ),
                })?;
        let updated_event_id =
            self.updated_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgOwnedAccountTransactionListOwnerRowError::MissingUserOwner,
                    "organization" => {
                        PgOwnedAccountTransactionListOwnerRowError::MissingOrganizationOwner
                    }
                    _ => PgOwnedAccountTransactionListOwnerRowError::UnknownOwnerType(
                        self.owner_type.clone(),
                    ),
                })?;
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgOwnedAccountTransactionListOwnerRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgOwnedAccountTransactionListOwnerRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgOwnedAccountTransactionListOwnerRowError> {
        value.map(Username::try_from).transpose().map_err(|error| {
            PgOwnedAccountTransactionListOwnerRowError::InvalidUsername(Box::new(error))
        })
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgOwnedAccountTransactionListOwnerRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOwnedAccountTransactionListOwnerRowError::InvalidUserDisplayName(Box::new(error))
            })
    }
}

impl TryFrom<PgOwnedAccountTransactionListOwnerRow> for OwnedAccountTransactionListOwner {
    type Error = PgOwnedAccountTransactionListOwnerRowError;

    fn try_from(row: PgOwnedAccountTransactionListOwnerRow) -> Result<Self, Self::Error> {
        let observation = row.observation()?;

        match row.owner_type.as_str() {
            "user" => Ok(Self::User(OwnedAccountTransactionListOwnerUserPart {
                id: UserId::try_from_uuid(row.owner_id).map_err(|error| {
                    PgOwnedAccountTransactionListOwnerRowError::InvalidUserOwnerId(Box::new(error))
                })?,
                username: PgOwnedAccountTransactionListOwnerRow::optional_username(
                    row.owner_user_username,
                )?,
                display_name: PgOwnedAccountTransactionListOwnerRow::optional_user_display_name(
                    row.owner_user_display_name,
                )?,
                picture: PgUserPictureRefColumns {
                    picture_type: row.owner_user_picture_type,
                    object_name: row.owner_user_picture_object_name,
                    external_url: row.owner_user_picture_external_url,
                }
                .into_picture()
                .map_err(|error| {
                    PgOwnedAccountTransactionListOwnerRowError::InvalidUserPicture(Box::new(error))
                })?,
                observation,
            })),
            "organization" => {
                let handle = row
                    .owner_organization_handle
                    .ok_or(PgOwnedAccountTransactionListOwnerRowError::MissingOrganizationOwner)?;
                let display_name = row
                    .owner_organization_display_name
                    .ok_or(PgOwnedAccountTransactionListOwnerRowError::MissingOrganizationOwner)?;

                Ok(Self::Organization(
                    OwnedAccountTransactionListOwnerOrganizationPart {
                        id: OrganizationId::try_from_uuid(row.owner_id).map_err(|error| {
                            PgOwnedAccountTransactionListOwnerRowError::InvalidOrganizationOwnerId(
                                Box::new(error),
                            )
                        })?,
                        handle: OrganizationHandle::try_from(handle).map_err(|error| {
                            PgOwnedAccountTransactionListOwnerRowError::InvalidOrganizationHandle(
                                Box::new(error),
                            )
                        })?,
                        display_name: OrganizationDisplayName::try_from(display_name).map_err(
                            |error| {
                                PgOwnedAccountTransactionListOwnerRowError::InvalidOrganizationDisplayName(
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
                            PgOwnedAccountTransactionListOwnerRowError::InvalidOrganizationPicture(
                                Box::new(error),
                            )
                        })?,
                        observation,
                    },
                ))
            }
            value => {
                Err(PgOwnedAccountTransactionListOwnerRowError::UnknownOwnerType(value.to_owned()))
            }
        }
    }
}
