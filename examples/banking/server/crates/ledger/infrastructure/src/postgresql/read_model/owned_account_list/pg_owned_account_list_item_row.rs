use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_ledger_application::{
    OwnedAccountListItem, OwnedAccountListItemCurrency, OwnedAccountListItemStatus,
};
use banking_ledger_domain::account::{AccountDescription, AccountId, AccountName};
use banking_ledger_domain::core::{CurrencyAmount, CurrencyCode, CurrencyDecimals};
use banking_ledger_domain::currency::CurrencyId;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_owned_account_list_item_row_error::PgOwnedAccountListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgOwnedAccountListItemRow {
    pub account_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency_id: Uuid,
    pub currency_code: String,
    pub currency_decimals: i16,
    pub currency_source_event_id: Uuid,
    pub currency_updated_event_id: Uuid,
    pub balance: String,
    pub reserved_balance: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgOwnedAccountListItemRow {
    fn observation(
        source_event_id: Uuid,
        updated_event_id: Uuid,
    ) -> Result<ReadModelObservation, PgOwnedAccountListItemRowError> {
        Ok(ReadModelObservation::new(
            EventId::try_from(source_event_id).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidSourceEventId(Box::new(error))
            })?,
            EventId::try_from(updated_event_id).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidUpdatedEventId(Box::new(error))
            })?,
        ))
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
        let currency = OwnedAccountListItemCurrency {
            id: CurrencyId::try_from_uuid(row.currency_id).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidCurrencyCode(Box::new(error))
            })?,
            code: CurrencyCode::try_from(row.currency_code).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidCurrencyCode(Box::new(error))
            })?,
            decimals: CurrencyDecimals::new(currency_decimals),
            observation: PgOwnedAccountListItemRow::observation(
                row.currency_source_event_id,
                row.currency_updated_event_id,
            )?,
        };

        Ok(Self {
            account_id: AccountId::try_from_uuid(row.account_id).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            name: AccountName::try_from(row.name).map_err(|error| {
                PgOwnedAccountListItemRowError::InvalidAccountName(Box::new(error))
            })?,
            description: row
                .description
                .map(AccountDescription::try_from)
                .transpose()
                .map_err(|error| {
                    PgOwnedAccountListItemRowError::InvalidAccountDescription(Box::new(error))
                })?,
            currency,
            balance: PgOwnedAccountListItemRow::amount(row.balance)?,
            reserved_balance: PgOwnedAccountListItemRow::amount(row.reserved_balance)?,
            status: PgOwnedAccountListItemRow::status(row.status)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: PgOwnedAccountListItemRow::observation(
                row.source_event_id,
                row.updated_event_id,
            )?,
        })
    }
}
