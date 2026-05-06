use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_application::{PublicAccountListItem, PublicAccountListItemCurrency};
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use sqlx::types::chrono::{DateTime, Utc};

use super::pg_public_account_list_item_row_error::PgPublicAccountListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgPublicAccountListItemRow {
    pub account_id: uuid::Uuid,
    pub owner_type: String,
    pub owner_id: uuid::Uuid,
    pub currency_id: uuid::Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
    pub created_at: DateTime<Utc>,
}

impl PgPublicAccountListItemRow {
    fn owner(
        owner_type: String,
        owner_id: uuid::Uuid,
    ) -> Result<AccountOwner, PgPublicAccountListItemRowError> {
        match owner_type.as_str() {
            "user" => Ok(AccountOwner::User(
                UserId::try_from_uuid(owner_id).map_err(|error| {
                    PgPublicAccountListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            "organization" => Ok(AccountOwner::Organization(
                OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                    PgPublicAccountListItemRowError::InvalidOwnerId(Box::new(error))
                })?,
            )),
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
            owner: PgPublicAccountListItemRow::owner(row.owner_type, row.owner_id)?,
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
            },
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}
