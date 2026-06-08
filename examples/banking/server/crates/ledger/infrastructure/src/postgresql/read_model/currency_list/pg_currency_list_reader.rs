use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyList, CurrencyListCriteria, CurrencyListCursor, CurrencyListItem,
    CurrencyListItemStatus, CurrencyListReader, CurrencyListReaderError, CurrencyListSortKey,
};
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};

use super::pg_currency_list_item_row::PgCurrencyListItemRow;

/// PostgreSQL-backed currency list reader.
pub struct PgCurrencyListReader;

impl PgCurrencyListReader {
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

impl Default for PgCurrencyListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyListReader for PgCurrencyListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: CurrencyListCriteria,
        cursor_options: Option<CursorOptions<CurrencyListSortKey, CurrencyListCursor>>,
        page_size: PageSize,
    ) -> Result<CurrencyList, CurrencyListReaderError> {
        let limit = i64::from(page_size.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id AS currency_id,
                i.owner_type,
                i.owner_id,
                u.username AS owner_user_username,
                u.display_name AS owner_user_display_name,
                u.picture_type AS owner_user_picture_type,
                u.picture_object_name AS owner_user_picture_object_name,
                u.picture_external_url AS owner_user_picture_external_url,
                o.handle AS owner_organization_handle,
                o.display_name AS owner_organization_display_name,
                o.picture_type AS owner_organization_picture_type,
                o.picture_object_name AS owner_organization_picture_object_name,
                o.picture_external_url AS owner_organization_picture_external_url,
                COALESCE(u.source_event_id, o.source_event_id) AS owner_source_event_id,
                COALESCE(u.updated_event_id, o.updated_event_id) AS owner_updated_event_id,
                i.symbol,
                i.name,
                i.decimals,
                i.description,
                i.image_type,
                i.image_object_name,
                i.image_external_url,
                i.supply::text AS supply,
                i.status,
                i.created_at,
                i.source_event_id,
                i.updated_event_id
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

        if let Some(status) = criteria.status {
            builder
                .push(" AND i.status = ")
                .push_bind(Self::status_name(status));
        }

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(CurrencyListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (CurrencyListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.created_at, i.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.currency_id.value())
                        .push(")");
                }
                (CurrencyListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.created_at, i.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.currency_id.value())
                        .push(")");
                }
                (CurrencyListSortKey::CurrencyId, SortDirection::Asc) => {
                    builder
                        .push(" AND i.id > ")
                        .push_bind(cursor.currency_id.value());
                }
                (CurrencyListSortKey::CurrencyId, SortDirection::Desc) => {
                    builder
                        .push(" AND i.id < ")
                        .push_bind(cursor.currency_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (CurrencyListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.created_at ASC, i.id ASC");
            }
            (CurrencyListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.created_at DESC, i.id DESC");
            }
            (CurrencyListSortKey::CurrencyId, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (CurrencyListSortKey::CurrencyId, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgCurrencyListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| CurrencyListReaderError::Persistence(Box::new(e)))?;

        let limit = page_size.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(CurrencyListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CurrencyListReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| CurrencyListCursor {
                created_at: item.created_at,
                currency_id: item.currency_id,
            })
        } else {
            None
        };

        Ok(CurrencyList { items, next_cursor })
    }
}
