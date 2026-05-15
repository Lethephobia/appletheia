use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{
    CurrencyListItem, CurrencyListItemOwner, CurrencyListItemOwnerOrganization,
    CurrencyListItemOwnerUser, CurrencyListItemStatus, ReadModelObservation,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use sqlx::types::chrono::{DateTime, Utc};

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_currency_list_item_row_error::PgCurrencyListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgCurrencyListItemRow {
    pub currency_id: uuid::Uuid,
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
    pub owner_source_event_id: Option<uuid::Uuid>,
    pub owner_updated_event_id: Option<uuid::Uuid>,
    pub symbol: String,
    pub name: String,
    pub decimals: i16,
    pub supply: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: uuid::Uuid,
    pub updated_event_id: uuid::Uuid,
}

impl PgCurrencyListItemRow {
    fn observation(
        source_event_id: uuid::Uuid,
        updated_event_id: uuid::Uuid,
    ) -> Result<ReadModelObservation, PgCurrencyListItemRowError> {
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgCurrencyListItemRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgCurrencyListItemRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
    }

    fn owner_observation(&self) -> Result<ReadModelObservation, PgCurrencyListItemRowError> {
        let source_event_id =
            self.owner_source_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgCurrencyListItemRowError::MissingUserOwner,
                    "organization" => PgCurrencyListItemRowError::MissingOrganizationOwner,
                    _ => PgCurrencyListItemRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        let updated_event_id =
            self.owner_updated_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgCurrencyListItemRowError::MissingUserOwner,
                    "organization" => PgCurrencyListItemRowError::MissingOrganizationOwner,
                    _ => PgCurrencyListItemRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        Self::observation(source_event_id, updated_event_id)
    }

    fn status(value: String) -> Result<CurrencyListItemStatus, PgCurrencyListItemRowError> {
        match value.as_str() {
            "active" => Ok(CurrencyListItemStatus::Active),
            "inactive" => Ok(CurrencyListItemStatus::Inactive),
            value => Err(PgCurrencyListItemRowError::UnknownStatus(value.to_owned())),
        }
    }

    fn amount(value: String) -> Result<CurrencyAmount, PgCurrencyListItemRowError> {
        let value = value
            .parse::<u128>()
            .map_err(PgCurrencyListItemRowError::InvalidCurrencyAmount)?;
        Ok(CurrencyAmount::new(value))
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgCurrencyListItemRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgCurrencyListItemRowError::InvalidUsername(Box::new(error)))
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgCurrencyListItemRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| PgCurrencyListItemRowError::InvalidUserDisplayName(Box::new(error)))
    }

    fn owner(&self) -> Result<CurrencyListItemOwner, PgCurrencyListItemRowError> {
        match self.owner_type.as_str() {
            "user" => Ok(CurrencyListItemOwner::User(CurrencyListItemOwnerUser {
                id: UserId::try_from_uuid(self.owner_id).map_err(|error| {
                    PgCurrencyListItemRowError::InvalidUserOwnerId(Box::new(error))
                })?,
                username: Self::optional_username(self.owner_user_username.clone())?,
                display_name: Self::optional_user_display_name(
                    self.owner_user_display_name.clone(),
                )?,
                picture: PgUserPictureRefColumns {
                    picture_type: self.owner_user_picture_type.clone(),
                    object_name: self.owner_user_picture_object_name.clone(),
                    external_url: self.owner_user_picture_external_url.clone(),
                }
                .into_picture()
                .map_err(|error| PgCurrencyListItemRowError::InvalidUserPicture(Box::new(error)))?,
                observation: self.owner_observation()?,
            })),
            "organization" => {
                let handle = self
                    .owner_organization_handle
                    .clone()
                    .ok_or(PgCurrencyListItemRowError::MissingOrganizationOwner)?;
                let display_name = self
                    .owner_organization_display_name
                    .clone()
                    .ok_or(PgCurrencyListItemRowError::MissingOrganizationOwner)?;

                Ok(CurrencyListItemOwner::Organization(
                    CurrencyListItemOwnerOrganization {
                        id: OrganizationId::try_from_uuid(self.owner_id).map_err(|error| {
                            PgCurrencyListItemRowError::InvalidOrganizationOwnerId(Box::new(error))
                        })?,
                        handle: OrganizationHandle::try_from(handle).map_err(|error| {
                            PgCurrencyListItemRowError::InvalidOrganizationHandle(Box::new(error))
                        })?,
                        display_name: OrganizationDisplayName::try_from(display_name).map_err(
                            |error| {
                                PgCurrencyListItemRowError::InvalidOrganizationDisplayName(
                                    Box::new(error),
                                )
                            },
                        )?,
                        picture: PgOrganizationPictureRefColumns {
                            picture_type: self.owner_organization_picture_type.clone(),
                            object_name: self.owner_organization_picture_object_name.clone(),
                            external_url: self.owner_organization_picture_external_url.clone(),
                        }
                        .into_picture()
                        .map_err(|error| {
                            PgCurrencyListItemRowError::InvalidOrganizationPicture(Box::new(error))
                        })?,
                        observation: self.owner_observation()?,
                    },
                ))
            }
            value => Err(PgCurrencyListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }
}

impl TryFrom<PgCurrencyListItemRow> for CurrencyListItem {
    type Error = PgCurrencyListItemRowError;

    fn try_from(row: PgCurrencyListItemRow) -> Result<Self, Self::Error> {
        let currency_decimals = u8::try_from(row.decimals).map_err(|error| {
            PgCurrencyListItemRowError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(Self {
            currency_id: CurrencyId::try_from_uuid(row.currency_id)
                .map_err(|error| PgCurrencyListItemRowError::InvalidCurrencyId(Box::new(error)))?,
            owner: row.owner()?,
            symbol: CurrencySymbol::try_from(row.symbol).map_err(|error| {
                PgCurrencyListItemRowError::InvalidCurrencySymbol(Box::new(error))
            })?,
            name: CurrencyName::try_from(row.name).map_err(|error| {
                PgCurrencyListItemRowError::InvalidCurrencyName(Box::new(error))
            })?,
            decimals: CurrencyDecimals::new(currency_decimals),
            supply: PgCurrencyListItemRow::amount(row.supply)?,
            status: PgCurrencyListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: PgCurrencyListItemRow::observation(
                row.source_event_id,
                row.updated_event_id,
            )?,
        })
    }
}
