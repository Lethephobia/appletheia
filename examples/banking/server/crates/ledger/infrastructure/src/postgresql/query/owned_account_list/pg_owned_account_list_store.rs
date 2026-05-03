use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
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

use super::pg_owned_account_list_store_error::PgOwnedAccountListStoreError;

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

    fn owner(
        owner_type: String,
        owner_id: uuid::Uuid,
    ) -> Result<AccountOwner, PgOwnedAccountListStoreError> {
        match owner_type.as_str() {
            "user" => Ok(AccountOwner::User(
                UserId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountListStoreError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            "organization" => Ok(AccountOwner::Organization(
                OrganizationId::try_from_uuid(owner_id).map_err(|error| {
                    PgOwnedAccountListStoreError::InvalidOwnerId(Box::new(error))
                })?,
            )),
            value => Err(PgOwnedAccountListStoreError::UnknownOwnerType(
                value.to_owned(),
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

    fn status(value: String) -> Result<AccountStatus, PgOwnedAccountListStoreError> {
        match value.as_str() {
            "active" => Ok(AccountStatus::Active),
            "frozen" => Ok(AccountStatus::Frozen),
            "closed" => Ok(AccountStatus::Closed),
            value => Err(PgOwnedAccountListStoreError::UnknownStatus(
                value.to_owned(),
            )),
        }
    }

    fn amount(value: String) -> Result<CurrencyAmount, PgOwnedAccountListStoreError> {
        let value = value
            .parse::<u128>()
            .map_err(PgOwnedAccountListStoreError::InvalidCurrencyAmount)?;
        Ok(CurrencyAmount::new(value))
    }

    fn item(
        row: sqlx::postgres::PgRow,
    ) -> Result<OwnedAccountListItem, PgOwnedAccountListStoreError> {
        let currency_decimals: i16 = row.get("currency_decimals");
        let currency_decimals = u8::try_from(currency_decimals).map_err(|error| {
            PgOwnedAccountListStoreError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(OwnedAccountListItem {
            id: AccountId::try_from_uuid(row.get("id"))
                .map_err(|error| PgOwnedAccountListStoreError::InvalidAccountId(Box::new(error)))?,
            owner: Self::owner(row.get("owner_type"), row.get("owner_id"))?,
            name: AccountName::try_from(row.get::<String, _>("name")).map_err(|error| {
                PgOwnedAccountListStoreError::InvalidAccountName(Box::new(error))
            })?,
            currency: OwnedAccountListItemCurrency {
                id: CurrencyId::try_from_uuid(row.get("currency_id")).map_err(|error| {
                    PgOwnedAccountListStoreError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(row.get::<String, _>("currency_symbol")).map_err(
                    |error| PgOwnedAccountListStoreError::InvalidCurrencySymbol(Box::new(error)),
                )?,
                name: CurrencyName::try_from(row.get::<String, _>("currency_name")).map_err(
                    |error| PgOwnedAccountListStoreError::InvalidCurrencyName(Box::new(error)),
                )?,
                decimals: CurrencyDecimals::new(currency_decimals),
            },
            balance: Self::amount(row.get("balance"))?,
            reserved_balance: Self::amount(row.get("reserved_balance"))?,
            status: Self::status(row.get("status"))?,
            created_at: EventOccurredAt::from(
                row.get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, _>("created_at"),
            ),
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
                a.created_at,
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

        let sort_key = query
            .cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OwnedAccountListSortKey::CreatedAt);
        let sort_direction = query
            .cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = query.cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (OwnedAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (a.created_at, a.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (a.created_at, a.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountListSortKey::Id, SortDirection::Asc) => {
                    builder.push(" AND a.id > ").push_bind(cursor.id.value());
                }
                (OwnedAccountListSortKey::Id, SortDirection::Desc) => {
                    builder.push(" AND a.id < ").push_bind(cursor.id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (OwnedAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY a.created_at ASC, a.id ASC");
            }
            (OwnedAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY a.created_at DESC, a.id DESC");
            }
            (OwnedAccountListSortKey::Id, SortDirection::Asc) => {
                builder.push(" ORDER BY a.id ASC");
            }
            (OwnedAccountListSortKey::Id, SortDirection::Desc) => {
                builder.push(" ORDER BY a.id DESC");
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| OwnedAccountListStoreError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| OwnedAccountListCursor {
                created_at: item.created_at,
                id: item.id,
            })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
