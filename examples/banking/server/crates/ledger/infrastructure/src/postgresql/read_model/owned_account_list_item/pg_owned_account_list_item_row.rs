use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_application::{
    OwnedAccountListItem, OwnedAccountListItemCurrency, OwnedAccountListItemStatus,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use sqlx::types::chrono::{DateTime, Utc};

use super::pg_owned_account_list_item_row_error::PgOwnedAccountListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountListItemRow {
    pub id: uuid::Uuid,
    pub owner_type: String,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub currency_id: uuid::Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
    pub balance: String,
    pub reserved_balance: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl PgOwnedAccountListItemRow {
    fn owner(
        owner_type: String,
        owner_id: uuid::Uuid,
    ) -> Result<AccountOwner, PgOwnedAccountListItemRowError> {
        match owner_type.as_str() {
            "user" => Ok(AccountOwner::User(
                UserId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            "organization" => Ok(AccountOwner::Organization(
                OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            value => Err(PgOwnedAccountListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }

    fn status(value: String) -> Result<OwnedAccountListItemStatus, PgOwnedAccountListItemRowError> {
        match value.as_str() {
            "active" => Ok(OwnedAccountListItemStatus::Active),
            "frozen" => Ok(OwnedAccountListItemStatus::Frozen),
            value => Err(PgOwnedAccountListItemRowError::UnknownStatus(
                value.to_owned(),
            )),
        }
    }

    fn amount(value: String) -> Result<CurrencyAmount, PgOwnedAccountListItemRowError> {
        let value = value
            .parse::<u128>()
            .map_err(PgOwnedAccountListItemRowError::InvalidCurrencyAmount)?;
        Ok(CurrencyAmount::new(value))
    }
}

impl TryFrom<PgOwnedAccountListItemRow> for OwnedAccountListItem {
    type Error = PgOwnedAccountListItemRowError;

    fn try_from(row: PgOwnedAccountListItemRow) -> Result<Self, Self::Error> {
        let currency_decimals = u8::try_from(row.currency_decimals).map_err(|error| {
            PgOwnedAccountListItemRowError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(Self {
            id: AccountId::try_from_uuid(row.id).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            owner: PgOwnedAccountListItemRow::owner(row.owner_type, row.owner_id)?,
            name: AccountName::try_from(row.name).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidAccountName(Box::new(error))
            })?,
            currency: OwnedAccountListItemCurrency {
                id: CurrencyId::try_from_uuid(row.currency_id).map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(row.currency_symbol).map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidCurrencySymbol(Box::new(error))
                })?,
                name: CurrencyName::try_from(row.currency_name).map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidCurrencyName(Box::new(error))
                })?,
                decimals: CurrencyDecimals::new(currency_decimals),
            },
            balance: PgOwnedAccountListItemRow::amount(row.balance)?,
            reserved_balance: PgOwnedAccountListItemRow::amount(row.reserved_balance)?,
            status: PgOwnedAccountListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}
