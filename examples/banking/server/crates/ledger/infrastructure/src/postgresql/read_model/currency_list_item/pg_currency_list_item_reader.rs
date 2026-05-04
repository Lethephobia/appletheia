use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyListItem, CurrencyListItemCursor, CurrencyListItemReader, CurrencyListItemReaderError,
    CurrencyListItemSortKey, CurrencyListItemStatus, CursorOptions, Page, PageLimit, SortDirection,
};
use sqlx::{Postgres, QueryBuilder};

use super::pg_currency_list_item_row::PgCurrencyListItemRow;

/// PostgreSQL-backed currency list item reader.
pub struct PgCurrencyListItemReader;

impl PgCurrencyListItemReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: CurrencyListItemStatus) -> &'static str {
        match status {
            CurrencyListItemStatus::Active => "active",
            CurrencyListItemStatus::Inactive => "inactive",
        }
    }
}

impl Default for PgCurrencyListItemReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyListItemReader for PgCurrencyListItemReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        status: Option<CurrencyListItemStatus>,
        cursor_options: Option<CursorOptions<CurrencyListItemSortKey, CurrencyListItemCursor>>,
        page_limit: PageLimit,
    ) -> Result<Page<CurrencyListItem, CurrencyListItemCursor>, CurrencyListItemReaderError> {
        let limit = i64::from(page_limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id,
                i.owner_type,
                i.owner_id,
                u.username AS owner_user_username,
                u.display_name AS owner_user_display_name,
                u.picture AS owner_user_picture,
                o.handle AS owner_organization_handle,
                o.display_name AS owner_organization_display_name,
                o.picture AS owner_organization_picture,
                i.symbol,
                i.name,
                i.decimals,
                i.supply::text AS supply,
                i.status,
                i.created_at
            FROM currency_list_items i
            LEFT JOIN currency_list_item_owner_users u
                   ON i.owner_type = 'user'
                  AND u.id = i.owner_id
            LEFT JOIN currency_list_item_owner_organizations o
                   ON i.owner_type = 'organization'
                  AND o.id = i.owner_id
            WHERE TRUE
            "#,
        );

        if let Some(status) = status {
            builder
                .push(" AND i.status = ")
                .push_bind(Self::status_name(status));
        }

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(CurrencyListItemSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (CurrencyListItemSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.created_at, i.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (CurrencyListItemSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.created_at, i.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (CurrencyListItemSortKey::Id, SortDirection::Asc) => {
                    builder.push(" AND i.id > ").push_bind(cursor.id.value());
                }
                (CurrencyListItemSortKey::Id, SortDirection::Desc) => {
                    builder.push(" AND i.id < ").push_bind(cursor.id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (CurrencyListItemSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.created_at ASC, i.id ASC");
            }
            (CurrencyListItemSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.created_at DESC, i.id DESC");
            }
            (CurrencyListItemSortKey::Id, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (CurrencyListItemSortKey::Id, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgCurrencyListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| CurrencyListItemReaderError::Persistence(Box::new(e)))?;

        let limit = page_limit.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(CurrencyListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CurrencyListItemReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| CurrencyListItemCursor {
                created_at: item.created_at,
                id: item.id,
            })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
