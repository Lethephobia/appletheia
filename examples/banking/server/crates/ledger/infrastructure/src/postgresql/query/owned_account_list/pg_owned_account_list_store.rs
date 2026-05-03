use std::io;

use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_application::{
    OwnedAccountListCursor, OwnedAccountListItem, OwnedAccountListItemCurrency,
    OwnedAccountListQuery, OwnedAccountListSortKey, OwnedAccountListStore,
    OwnedAccountListStoreError, Page, SortDirection,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use sqlx::{Postgres, QueryBuilder, Row};

/// PostgreSQL-backed account list store.
pub struct PgOwnedAccountListStore;

impl PgOwnedAccountListStore {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: AccountOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            AccountOwner::User(user_id) => ("user", user_id.value()),
            AccountOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn owner(owner_type: String, owner_id: uuid::Uuid) -> Result<AccountOwner, io::Error> {
        match owner_type.as_str() {
            "user" => Ok(AccountOwner::User(
                UserId::try_from_uuid(owner_id).map_err(|error| {
                    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
                })?,
            )),
            "organization" => Ok(AccountOwner::Organization(
                OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
                })?,
            )),
            value => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown account owner type: {value}"),
            )),
        }
    }

    fn status_name(status: AccountStatus) -> &'static str {
        match status {
            AccountStatus::Active => "active",
            AccountStatus::Frozen => "frozen",
            AccountStatus::Closed => "closed",
        }
    }

    fn status(value: String) -> Result<AccountStatus, io::Error> {
        match value.as_str() {
            "active" => Ok(AccountStatus::Active),
            "frozen" => Ok(AccountStatus::Frozen),
            "closed" => Ok(AccountStatus::Closed),
            value => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown account status: {value}"),
            )),
        }
    }

    fn amount(value: String) -> Result<CurrencyAmount, io::Error> {
        let value = value
            .parse::<u128>()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
        Ok(CurrencyAmount::new(value))
    }

    fn item(
        row: sqlx::postgres::PgRow,
    ) -> Result<OwnedAccountListItem, OwnedAccountListStoreError> {
        let currency_decimals: i16 = row.get("currency_decimals");
        let currency_decimals = u8::try_from(currency_decimals).map_err(|error| {
            OwnedAccountListStoreError::Persistence(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                error.to_string(),
            )))
        })?;

        Ok(OwnedAccountListItem {
            id: AccountId::try_from_uuid(row.get("id"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
            owner: Self::owner(row.get("owner_type"), row.get("owner_id"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
            name: AccountName::try_from(row.get::<String, _>("name"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
            currency: OwnedAccountListItemCurrency {
                id: CurrencyId::try_from_uuid(row.get("currency_id"))
                    .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
                symbol: CurrencySymbol::try_from(row.get::<String, _>("currency_symbol"))
                    .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
                name: CurrencyName::try_from(row.get::<String, _>("currency_name"))
                    .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
                decimals: CurrencyDecimals::new(currency_decimals),
            },
            balance: Self::amount(row.get("balance"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
            reserved_balance: Self::amount(row.get("reserved_balance"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
            status: Self::status(row.get("status"))
                .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?,
        })
    }
}

impl Default for PgOwnedAccountListStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountListStore for PgOwnedAccountListStore {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        query: &OwnedAccountListQuery,
    ) -> Result<Page<OwnedAccountListItem, OwnedAccountListCursor>, OwnedAccountListStoreError>
    {
        let (owner_type, owner_id) = Self::owner_parts(query.owner);
        let limit = i64::from(query.limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                a.id,
                a.owner_type,
                a.owner_id,
                a.name,
                a.currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals,
                a.balance::text AS balance,
                a.reserved_balance::text AS reserved_balance,
                a.status
            FROM accounts a
            INNER JOIN currencies c ON c.id = a.currency_id
            WHERE a.owner_type =
            "#,
        );

        builder
            .push_bind(owner_type)
            .push(" AND a.owner_id = ")
            .push_bind(owner_id);

        if let Some(currency_id) = query.currency_id {
            builder
                .push(" AND a.currency_id = ")
                .push_bind(currency_id.value());
        }

        if let Some(status) = query.status {
            builder
                .push(" AND a.status = ")
                .push_bind(Self::status_name(status));
        }

        if let Some(cursor_options) = query.cursor_options {
            if let Some(cursor) = cursor_options.cursor {
                match cursor_options.sort_direction {
                    SortDirection::Asc => {
                        builder.push(" AND a.id > ").push_bind(cursor.id.value());
                    }
                    SortDirection::Desc => {
                        builder.push(" AND a.id < ").push_bind(cursor.id.value());
                    }
                }
            }

            match (cursor_options.sort_key, cursor_options.sort_direction) {
                (OwnedAccountListSortKey::Id, SortDirection::Asc) => {
                    builder.push(" ORDER BY a.id ASC");
                }
                (OwnedAccountListSortKey::Id, SortDirection::Desc) => {
                    builder.push(" ORDER BY a.id DESC");
                }
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?;

        let limit = query.limit.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(Self::item)
            .collect::<Result<Vec<_>, _>>()?;
        let next_cursor = if has_next && query.cursor_options.is_some() {
            items
                .last()
                .map(|item| OwnedAccountListCursor { id: item.id })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
