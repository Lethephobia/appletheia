use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{
    OwnedAccountTransactionId, OwnedAccountTransactionListItem,
    OwnedAccountTransactionListItemCounterpartyAccount,
    OwnedAccountTransactionListItemCounterpartyAccountOwner,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUser,
    OwnedAccountTransactionListItemCurrency, OwnedAccountTransactionListItemDirection,
    OwnedAccountTransactionListItemKind, OwnedAccountTransactionListItemStatus,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::transfer::TransferId;
use banking_shared_kernel_application::read_model::ReadModelObservation;
use sqlx::types::chrono::{DateTime, Utc};

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;
use super::pg_owned_account_transaction_list_item_row_error::PgOwnedAccountTransactionListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountTransactionListItemRow {
    pub transaction_id: uuid::Uuid,
    pub transfer_id: Option<uuid::Uuid>,
    pub account_id: uuid::Uuid,
    pub counterparty_account_id: Option<uuid::Uuid>,
    pub counterparty_owner_type: Option<String>,
    pub counterparty_owner_id: Option<uuid::Uuid>,
    pub counterparty_owner_user_username: Option<String>,
    pub counterparty_owner_user_display_name: Option<String>,
    pub counterparty_owner_user_picture_type: Option<String>,
    pub counterparty_owner_user_picture_object_name: Option<String>,
    pub counterparty_owner_user_picture_external_url: Option<String>,
    pub counterparty_owner_organization_handle: Option<String>,
    pub counterparty_owner_organization_display_name: Option<String>,
    pub counterparty_owner_organization_picture_type: Option<String>,
    pub counterparty_owner_organization_picture_object_name: Option<String>,
    pub counterparty_owner_organization_picture_external_url: Option<String>,
    pub counterparty_owner_source_event_id: Option<uuid::Uuid>,
    pub counterparty_owner_updated_event_id: Option<uuid::Uuid>,
    pub counterparty_account_source_event_id: Option<uuid::Uuid>,
    pub counterparty_account_updated_event_id: Option<uuid::Uuid>,
    pub currency_id: uuid::Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
    pub currency_source_event_id: uuid::Uuid,
    pub currency_updated_event_id: uuid::Uuid,
    pub amount: String,
    pub direction: String,
    pub kind: String,
    pub status: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: uuid::Uuid,
    pub updated_event_id: uuid::Uuid,
}

impl PgOwnedAccountTransactionListItemRow {
    fn observation(
        source_event_id: uuid::Uuid,
        updated_event_id: uuid::Uuid,
    ) -> Result<ReadModelObservation, PgOwnedAccountTransactionListItemRowError> {
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
    }

    fn direction(
        value: String,
    ) -> Result<OwnedAccountTransactionListItemDirection, PgOwnedAccountTransactionListItemRowError>
    {
        match value.as_str() {
            "incoming" => Ok(OwnedAccountTransactionListItemDirection::Incoming),
            "outgoing" => Ok(OwnedAccountTransactionListItemDirection::Outgoing),
            value => Err(PgOwnedAccountTransactionListItemRowError::UnknownDirection(
                value.to_owned(),
            )),
        }
    }

    fn kind(
        row: &Self,
    ) -> Result<OwnedAccountTransactionListItemKind, PgOwnedAccountTransactionListItemRowError>
    {
        match row.kind.as_str() {
            "deposit" => {
                Self::ensure_no_transfer_attributes(row)?;
                Ok(OwnedAccountTransactionListItemKind::Deposit)
            }
            "withdrawal" => {
                Self::ensure_no_transfer_attributes(row)?;
                Ok(OwnedAccountTransactionListItemKind::Withdrawal)
            }
            "transfer" => Ok(OwnedAccountTransactionListItemKind::Transfer {
                transfer_id: Self::transfer_id(row.transfer_id)?,
                counterparty_account: Box::new(Self::counterparty_account(row)?),
            }),
            "currency_issuance" => {
                Self::ensure_no_transfer_attributes(row)?;
                Ok(OwnedAccountTransactionListItemKind::CurrencyIssuance)
            }
            value => Err(PgOwnedAccountTransactionListItemRowError::UnknownKind(
                value.to_owned(),
            )),
        }
    }

    fn transfer_id(
        value: Option<uuid::Uuid>,
    ) -> Result<TransferId, PgOwnedAccountTransactionListItemRowError> {
        let value =
            value.ok_or(PgOwnedAccountTransactionListItemRowError::MissingTransferAttributes)?;
        TransferId::try_from_uuid(value).map_err(|error| {
            PgOwnedAccountTransactionListItemRowError::InvalidTransferId(Box::new(error))
        })
    }

    fn counterparty_account(
        row: &Self,
    ) -> Result<
        OwnedAccountTransactionListItemCounterpartyAccount,
        PgOwnedAccountTransactionListItemRowError,
    > {
        let account_id = row
            .counterparty_account_id
            .ok_or(PgOwnedAccountTransactionListItemRowError::MissingTransferAttributes)?;

        Ok(OwnedAccountTransactionListItemCounterpartyAccount {
            id: AccountId::try_from_uuid(account_id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            owner: Self::counterparty_account_owner(row)?,
            observation: Self::observation(
                row.counterparty_account_source_event_id.ok_or(
                    PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountSource,
                )?,
                row.counterparty_account_updated_event_id.ok_or(
                    PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountSource,
                )?,
            )?,
        })
    }

    fn counterparty_account_owner(
        row: &Self,
    ) -> Result<
        OwnedAccountTransactionListItemCounterpartyAccountOwner,
        PgOwnedAccountTransactionListItemRowError,
    > {
        let owner_type = row
            .counterparty_owner_type
            .as_deref()
            .ok_or(PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwner)?;
        let owner_id = row
            .counterparty_owner_id
            .ok_or(PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwner)?;

        match owner_type {
            "user" => Ok(
                OwnedAccountTransactionListItemCounterpartyAccountOwner::User(
                    OwnedAccountTransactionListItemCounterpartyAccountOwnerUser {
                        id: UserId::try_from_uuid(owner_id).map_err(|error| {
                            PgOwnedAccountTransactionListItemRowError::InvalidUserOwnerId(Box::new(
                                error,
                            ))
                        })?,
                        username: Self::optional_username(
                            row.counterparty_owner_user_username.clone(),
                        )?,
                        display_name: Self::optional_user_display_name(
                            row.counterparty_owner_user_display_name.clone(),
                        )?,
                        picture: PgUserPictureRefColumns {
                            picture_type: row.counterparty_owner_user_picture_type.clone(),
                            object_name: row.counterparty_owner_user_picture_object_name.clone(),
                            external_url: row.counterparty_owner_user_picture_external_url.clone(),
                        }
                        .into_picture()
                        .map_err(|error| {
                            PgOwnedAccountTransactionListItemRowError::InvalidUserPicture(Box::new(
                                error,
                            ))
                        })?,
                        observation: Self::observation(
                            row.counterparty_owner_source_event_id.ok_or(
                                PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerSource,
                            )?,
                            row.counterparty_owner_updated_event_id.ok_or(
                                PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerSource,
                            )?,
                        )?,
                    },
                ),
            ),
            "organization" => {
                let handle = row
                    .counterparty_owner_organization_handle
                    .clone()
                    .ok_or(PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerOrganization)?;
                let display_name = row
                    .counterparty_owner_organization_display_name
                    .clone()
                    .ok_or(PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerOrganization)?;

                Ok(OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(
                    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization {
                        id: OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                            PgOwnedAccountTransactionListItemRowError::InvalidOrganizationOwnerId(
                                Box::new(error),
                            )
                        })?,
                        handle: OrganizationHandle::try_from(handle).map_err(|error| {
                            PgOwnedAccountTransactionListItemRowError::InvalidOrganizationHandle(
                                Box::new(error),
                            )
                        })?,
                        display_name: OrganizationDisplayName::try_from(display_name).map_err(
                            |error| {
                                PgOwnedAccountTransactionListItemRowError::InvalidOrganizationDisplayName(
                                    Box::new(error),
                                )
                            },
                        )?,
                        picture: PgOrganizationPictureRefColumns {
                            picture_type: row
                                .counterparty_owner_organization_picture_type
                                .clone(),
                            object_name: row
                                .counterparty_owner_organization_picture_object_name
                                .clone(),
                            external_url: row
                                .counterparty_owner_organization_picture_external_url
                                .clone(),
                        }
                        .into_picture()
                        .map_err(|error| {
                            PgOwnedAccountTransactionListItemRowError::InvalidOrganizationPicture(
                                Box::new(error),
                            )
                        })?,
                        observation: Self::observation(
                            row.counterparty_owner_source_event_id.ok_or(
                                PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerSource,
                            )?,
                            row.counterparty_owner_updated_event_id.ok_or(
                                PgOwnedAccountTransactionListItemRowError::MissingCounterpartyAccountOwnerSource,
                            )?,
                        )?,
                    },
                ))
            }
            value => Err(PgOwnedAccountTransactionListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgOwnedAccountTransactionListItemRowError> {
        value.map(Username::try_from).transpose().map_err(|error| {
            PgOwnedAccountTransactionListItemRowError::InvalidUsername(Box::new(error))
        })
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgOwnedAccountTransactionListItemRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidUserDisplayName(Box::new(error))
            })
    }

    fn ensure_no_transfer_attributes(
        row: &Self,
    ) -> Result<(), PgOwnedAccountTransactionListItemRowError> {
        if row.transfer_id.is_some()
            || row.counterparty_account_id.is_some()
            || row.counterparty_account_source_event_id.is_some()
            || row.counterparty_account_updated_event_id.is_some()
            || row.counterparty_owner_type.is_some()
            || row.counterparty_owner_id.is_some()
            || row.counterparty_owner_source_event_id.is_some()
            || row.counterparty_owner_updated_event_id.is_some()
        {
            return Err(PgOwnedAccountTransactionListItemRowError::UnexpectedTransferAttributes);
        }

        Ok(())
    }

    fn status(
        value: String,
    ) -> Result<OwnedAccountTransactionListItemStatus, PgOwnedAccountTransactionListItemRowError>
    {
        match value.as_str() {
            "pending" => Ok(OwnedAccountTransactionListItemStatus::Pending),
            "completed" => Ok(OwnedAccountTransactionListItemStatus::Completed),
            "failed" => Ok(OwnedAccountTransactionListItemStatus::Failed),
            "requires_review" => Ok(OwnedAccountTransactionListItemStatus::RequiresReview),
            value => Err(PgOwnedAccountTransactionListItemRowError::UnknownStatus(
                value.to_owned(),
            )),
        }
    }

    fn amount(value: String) -> Result<CurrencyAmount, PgOwnedAccountTransactionListItemRowError> {
        let value = value
            .parse::<u128>()
            .map_err(PgOwnedAccountTransactionListItemRowError::InvalidCurrencyAmount)?;
        Ok(CurrencyAmount::new(value))
    }
}

impl TryFrom<PgOwnedAccountTransactionListItemRow> for OwnedAccountTransactionListItem {
    type Error = PgOwnedAccountTransactionListItemRowError;

    fn try_from(row: PgOwnedAccountTransactionListItemRow) -> Result<Self, Self::Error> {
        let currency_decimals = u8::try_from(row.currency_decimals).map_err(|error| {
            PgOwnedAccountTransactionListItemRowError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(Self {
            transaction_id: OwnedAccountTransactionId::from(row.transaction_id),
            account_id: AccountId::try_from_uuid(row.account_id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            currency: OwnedAccountTransactionListItemCurrency {
                id: CurrencyId::try_from_uuid(row.currency_id).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(row.currency_symbol.clone()).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencySymbol(Box::new(
                        error,
                    ))
                })?,
                name: CurrencyName::try_from(row.currency_name.clone()).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencyName(Box::new(error))
                })?,
                decimals: CurrencyDecimals::new(currency_decimals),
                observation: PgOwnedAccountTransactionListItemRow::observation(
                    row.currency_source_event_id,
                    row.currency_updated_event_id,
                )?,
            },
            amount: PgOwnedAccountTransactionListItemRow::amount(row.amount.clone())?,
            direction: PgOwnedAccountTransactionListItemRow::direction(row.direction.clone())?,
            kind: PgOwnedAccountTransactionListItemRow::kind(&row)?,
            status: PgOwnedAccountTransactionListItemRow::status(row.status.clone())?,
            occurred_at: EventOccurredAt::from(row.occurred_at),
            created_at: EventOccurredAt::from(row.created_at),
            observation: PgOwnedAccountTransactionListItemRow::observation(
                row.source_event_id,
                row.updated_event_id,
            )?,
        })
    }
}
