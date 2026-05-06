use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CursorOptions, OwnedAccountListItem, OwnedAccountListItemCriteria, OwnedAccountListItemCursor,
    OwnedAccountListItemReader, OwnedAccountListItemReaderError, OwnedAccountListItemSortKey,
    OwnedAccountListItemStatus, Page, PageLimit, SortDirection,
};
use banking_ledger_domain::account::AccountOwner;
use sqlx::{Postgres, QueryBuilder};

use super::pg_owned_account_list_item_row::PgOwnedAccountListItemRow;

/// PostgreSQL-backed owned account list item reader.
pub struct PgOwnedAccountListItemReader;

impl PgOwnedAccountListItemReader {
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

    fn status_name(status: OwnedAccountListItemStatus) -> &'static str {
        match status {
            OwnedAccountListItemStatus::Active => "active",
            OwnedAccountListItemStatus::Frozen => "frozen",
        }
    }
}

impl Default for PgOwnedAccountListItemReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountListItemReader for PgOwnedAccountListItemReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountListItemCriteria,
        cursor_options: Option<
            CursorOptions<OwnedAccountListItemSortKey, OwnedAccountListItemCursor>,
        >,
        page_limit: PageLimit,
    ) -> Result<
        Page<OwnedAccountListItem, OwnedAccountListItemCursor>,
        OwnedAccountListItemReaderError,
    > {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let limit = i64::from(page_limit.value()) + 1;

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
                a.status,
                a.created_at
            FROM owned_account_list_items a
            INNER JOIN owned_account_list_item_currencies c ON c.id = a.currency_id
            WHERE a.owner_type =
            "#,
        );

        builder
            .push_bind(owner_type)
            .push(" AND a.owner_id = ")
            .push_bind(owner_id);

        if let Some(currency_id) = criteria.currency_id {
            builder
                .push(" AND a.currency_id = ")
                .push_bind(currency_id.value());
        }

        if let Some(status) = criteria.status {
            builder
                .push(" AND a.status = ")
                .push_bind(Self::status_name(status));
        }

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OwnedAccountListItemSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (OwnedAccountListItemSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (a.created_at, a.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountListItemSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (a.created_at, a.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountListItemSortKey::Id, SortDirection::Asc) => {
                    builder.push(" AND a.id > ").push_bind(cursor.id.value());
                }
                (OwnedAccountListItemSortKey::Id, SortDirection::Desc) => {
                    builder.push(" AND a.id < ").push_bind(cursor.id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (OwnedAccountListItemSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY a.created_at ASC, a.id ASC");
            }
            (OwnedAccountListItemSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY a.created_at DESC, a.id DESC");
            }
            (OwnedAccountListItemSortKey::Id, SortDirection::Asc) => {
                builder.push(" ORDER BY a.id ASC");
            }
            (OwnedAccountListItemSortKey::Id, SortDirection::Desc) => {
                builder.push(" ORDER BY a.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgOwnedAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountListItemReaderError::Persistence(Box::new(e)))?;

        let limit = page_limit.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(OwnedAccountListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| OwnedAccountListItemReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| OwnedAccountListItemCursor {
                created_at: item.created_at,
                id: item.id,
            })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
