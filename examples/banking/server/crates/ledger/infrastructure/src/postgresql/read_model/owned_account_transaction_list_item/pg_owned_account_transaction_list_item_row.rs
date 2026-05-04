use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_application::{
    OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCounterpartyAccount,
    OwnedAccountTransactionListItemCounterpartyAccountOwner,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganization,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUser,
    OwnedAccountTransactionListItemCurrency, OwnedAccountTransactionListItemDirection,
    OwnedAccountTransactionListItemKind, OwnedAccountTransactionListItemStatus,
};
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::transfer::TransferId;
use sqlx::types::chrono::{DateTime, Utc};

use super::pg_owned_account_transaction_list_item_row_error::PgOwnedAccountTransactionListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountTransactionListItemRow {
    pub id: uuid::Uuid,
    pub transfer_id: Option<uuid::Uuid>,
    pub owner_type: String,
    pub owner_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub counterparty_account_id: Option<uuid::Uuid>,
    pub counterparty_owner_type: Option<String>,
    pub counterparty_owner_id: Option<uuid::Uuid>,
    pub counterparty_owner_user_username: Option<String>,
    pub counterparty_owner_user_display_name: Option<String>,
    pub counterparty_owner_user_picture: Option<sqlx::types::Json<UserPictureRef>>,
    pub counterparty_owner_organization_handle: Option<String>,
    pub counterparty_owner_organization_display_name: Option<String>,
    pub counterparty_owner_organization_picture: Option<sqlx::types::Json<OrganizationPictureRef>>,
    pub currency_id: uuid::Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
    pub amount: String,
    pub direction: String,
    pub kind: String,
    pub status: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl PgOwnedAccountTransactionListItemRow {
    fn owner(
        owner_type: String,
        owner_id: uuid::Uuid,
    ) -> Result<AccountOwner, PgOwnedAccountTransactionListItemRowError> {
        match owner_type.as_str() {
            "user" => Ok(AccountOwner::User(
                UserId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            "organization" => Ok(AccountOwner::Organization(
                OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            value => Err(PgOwnedAccountTransactionListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
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
                counterparty_account: Self::counterparty_account(row)?,
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
            "user" => Ok(OwnedAccountTransactionListItemCounterpartyAccountOwner::User(
                OwnedAccountTransactionListItemCounterpartyAccountOwnerUser {
                    id: UserId::try_from_uuid(owner_id).map_err(|error| {
                        PgOwnedAccountTransactionListItemRowError::InvalidUserOwnerId(Box::new(
                            error,
                        ))
                    })?,
                    username: Self::optional_username(row.counterparty_owner_user_username.clone())?,
                    display_name: Self::optional_user_display_name(
                        row.counterparty_owner_user_display_name.clone(),
                    )?,
                    picture: row
                        .counterparty_owner_user_picture
                        .clone()
                        .map(|value| value.0),
                },
            )),
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
                        picture: row
                            .counterparty_owner_organization_picture
                            .clone()
                            .map(|value| value.0),
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
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| {
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
            || row.counterparty_owner_type.is_some()
            || row.counterparty_owner_id.is_some()
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
            id: EventId::try_from(row.id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidEventId(Box::new(error))
            })?,
            owner: PgOwnedAccountTransactionListItemRow::owner(row.owner_type.clone(), row.owner_id)?,
            account_id: AccountId::try_from_uuid(row.account_id).map_err(|error| {
                PgOwnedAccountTransactionListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            currency: OwnedAccountTransactionListItemCurrency {
                id: CurrencyId::try_from_uuid(row.currency_id).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(row.currency_symbol.clone()).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencySymbol(Box::new(error))
                })?,
                name: CurrencyName::try_from(row.currency_name.clone()).map_err(|error| {
                    PgOwnedAccountTransactionListItemRowError::InvalidCurrencyName(Box::new(error))
                })?,
                decimals: CurrencyDecimals::new(currency_decimals),
            },
            amount: PgOwnedAccountTransactionListItemRow::amount(row.amount.clone())?,
            direction: PgOwnedAccountTransactionListItemRow::direction(row.direction.clone())?,
            kind: PgOwnedAccountTransactionListItemRow::kind(&row)?,
            status: PgOwnedAccountTransactionListItemRow::status(row.status.clone())?,
            occurred_at: EventOccurredAt::from(row.occurred_at),
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}
