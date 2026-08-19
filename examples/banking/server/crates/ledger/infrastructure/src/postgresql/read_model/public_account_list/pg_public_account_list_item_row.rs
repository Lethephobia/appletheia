use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{
    PublicAccountListItem, PublicAccountListItemCurrency, PublicAccountListItemOwner,
    PublicAccountListItemOwnerOrganization, PublicAccountListItemOwnerUser,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_public_account_list_item_row_error::PgPublicAccountListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgPublicAccountListItemRow {
    pub account_id: Uuid,
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
    pub owner_source_event_id: Option<Uuid>,
    pub owner_updated_event_id: Option<Uuid>,
    pub currency_id: Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
    pub currency_mint_account_address: Option<String>,
    pub currency_source_event_id: Uuid,
    pub currency_updated_event_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgPublicAccountListItemRow {
    fn observation(
        source_event_id: Uuid,
        updated_event_id: Uuid,
    ) -> Result<ReadModelObservation, PgPublicAccountListItemRowError> {
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgPublicAccountListItemRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgPublicAccountListItemRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
    }

    fn owner_observation(&self) -> Result<ReadModelObservation, PgPublicAccountListItemRowError> {
        let source_event_id =
            self.owner_source_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgPublicAccountListItemRowError::MissingUserOwner,
                    "organization" => PgPublicAccountListItemRowError::MissingOrganizationOwner,
                    _ => PgPublicAccountListItemRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        let updated_event_id =
            self.owner_updated_event_id
                .ok_or_else(|| match self.owner_type.as_str() {
                    "user" => PgPublicAccountListItemRowError::MissingUserOwner,
                    "organization" => PgPublicAccountListItemRowError::MissingOrganizationOwner,
                    _ => PgPublicAccountListItemRowError::UnknownOwnerType(self.owner_type.clone()),
                })?;
        Self::observation(source_event_id, updated_event_id)
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgPublicAccountListItemRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgPublicAccountListItemRowError::InvalidUsername(Box::new(error)))
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgPublicAccountListItemRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgPublicAccountListItemRowError::InvalidUserDisplayName(Box::new(error))
            })
    }

    fn owner(&self) -> Result<PublicAccountListItemOwner, PgPublicAccountListItemRowError> {
        match self.owner_type.as_str() {
            "user" => Ok(PublicAccountListItemOwner::User(
                PublicAccountListItemOwnerUser {
                    id: UserId::try_from_uuid(self.owner_id).map_err(|error| {
                        PgPublicAccountListItemRowError::InvalidUserOwnerId(Box::new(error))
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
                    .map_err(|error| {
                        PgPublicAccountListItemRowError::InvalidUserPicture(Box::new(error))
                    })?,
                    observation: self.owner_observation()?,
                },
            )),
            "organization" => {
                let handle = self
                    .owner_organization_handle
                    .clone()
                    .ok_or(PgPublicAccountListItemRowError::MissingOrganizationOwner)?;
                let display_name = self
                    .owner_organization_display_name
                    .clone()
                    .ok_or(PgPublicAccountListItemRowError::MissingOrganizationOwner)?;

                Ok(PublicAccountListItemOwner::Organization(
                    PublicAccountListItemOwnerOrganization {
                        id: OrganizationId::try_from_uuid(self.owner_id).map_err(|error| {
                            PgPublicAccountListItemRowError::InvalidOrganizationOwnerId(Box::new(
                                error,
                            ))
                        })?,
                        handle: OrganizationHandle::try_from(handle).map_err(|error| {
                            PgPublicAccountListItemRowError::InvalidOrganizationHandle(Box::new(
                                error,
                            ))
                        })?,
                        display_name: OrganizationDisplayName::try_from(display_name).map_err(
                            |error| {
                                PgPublicAccountListItemRowError::InvalidOrganizationDisplayName(
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
                            PgPublicAccountListItemRowError::InvalidOrganizationPicture(Box::new(
                                error,
                            ))
                        })?,
                        observation: self.owner_observation()?,
                    },
                ))
            }
            value => Err(PgPublicAccountListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }
}

impl TryFrom<PgPublicAccountListItemRow> for PublicAccountListItem {
    type Error = PgPublicAccountListItemRowError;

    fn try_from(row: PgPublicAccountListItemRow) -> Result<Self, Self::Error> {
        let currency_decimals = u8::try_from(row.currency_decimals).map_err(|error| {
            PgPublicAccountListItemRowError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(Self {
            account_id: AccountId::try_from_uuid(row.account_id).map_err(|error| {
                PgPublicAccountListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            owner: row.owner()?,
            currency: PublicAccountListItemCurrency {
                id: CurrencyId::try_from_uuid(row.currency_id).map_err(|error| {
                    PgPublicAccountListItemRowError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(row.currency_symbol).map_err(|error| {
                    PgPublicAccountListItemRowError::InvalidCurrencySymbol(Box::new(error))
                })?,
                name: CurrencyName::try_from(row.currency_name).map_err(|error| {
                    PgPublicAccountListItemRowError::InvalidCurrencyName(Box::new(error))
                })?,
                decimals: CurrencyDecimals::new(currency_decimals),
                mint_account_address: row
                    .currency_mint_account_address
                    .map(MintAccountAddress::try_from)
                    .transpose()
                    .map_err(|error| {
                        PgPublicAccountListItemRowError::InvalidMintAccountAddress(Box::new(error))
                    })?,
                observation: PgPublicAccountListItemRow::observation(
                    row.currency_source_event_id,
                    row.currency_updated_event_id,
                )?,
            },
            created_at: EventOccurredAt::from(row.created_at),
            observation: PgPublicAccountListItemRow::observation(
                row.source_event_id,
                row.updated_event_id,
            )?,
        })
    }
}
