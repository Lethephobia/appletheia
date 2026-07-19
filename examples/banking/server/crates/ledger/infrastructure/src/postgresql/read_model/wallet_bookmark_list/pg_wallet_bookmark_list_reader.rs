use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    WalletBookmarkList, WalletBookmarkListCriteria, WalletBookmarkListCursor,
    WalletBookmarkListItem, WalletBookmarkListReader, WalletBookmarkListReaderError,
    WalletBookmarkListSortKey,
};
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use super::pg_wallet_bookmark_list_item_row::PgWalletBookmarkListItemRow;

/// PostgreSQL-backed wallet bookmark list reader.
pub struct PgWalletBookmarkListReader;

impl PgWalletBookmarkListReader {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: WalletBookmarkOwner) -> (&'static str, Uuid) {
        match owner {
            WalletBookmarkOwner::User(user_id) => ("user", user_id.value()),
            WalletBookmarkOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }
}

impl Default for PgWalletBookmarkListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletBookmarkListReader for PgWalletBookmarkListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: WalletBookmarkOwner,
        _criteria: WalletBookmarkListCriteria,
        cursor_options: Option<CursorOptions<WalletBookmarkListSortKey, WalletBookmarkListCursor>>,
        limit: PageSize,
    ) -> Result<WalletBookmarkList, WalletBookmarkListReaderError> {
        let query_limit = i64::from(limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id AS wallet_bookmark_id,
                owner_type,
                owner_id,
                display_name,
                description,
                token_account_owner_address,
                created_at,
                source_event_id,
                updated_event_id
            FROM wallet_bookmark_list_items
            WHERE owner_type =
            "#,
        );

        let (owner_type, owner_id) = Self::owner_parts(owner);
        builder
            .push_bind(owner_type)
            .push(" AND owner_id = ")
            .push_bind(owner_id);

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(WalletBookmarkListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (WalletBookmarkListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (created_at, id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.wallet_bookmark_id.value())
                        .push(")");
                }
                (WalletBookmarkListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (created_at, id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.wallet_bookmark_id.value())
                        .push(")");
                }
                (WalletBookmarkListSortKey::WalletBookmarkId, SortDirection::Asc) => {
                    builder
                        .push(" AND id > ")
                        .push_bind(cursor.wallet_bookmark_id.value());
                }
                (WalletBookmarkListSortKey::WalletBookmarkId, SortDirection::Desc) => {
                    builder
                        .push(" AND id < ")
                        .push_bind(cursor.wallet_bookmark_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (WalletBookmarkListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY created_at ASC, id ASC");
            }
            (WalletBookmarkListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY created_at DESC, id DESC");
            }
            (WalletBookmarkListSortKey::WalletBookmarkId, SortDirection::Asc) => {
                builder.push(" ORDER BY id ASC");
            }
            (WalletBookmarkListSortKey::WalletBookmarkId, SortDirection::Desc) => {
                builder.push(" ORDER BY id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgWalletBookmarkListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| WalletBookmarkListReaderError::Persistence(Box::new(e)))?;

        let page_limit = limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(WalletBookmarkListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| WalletBookmarkListReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| WalletBookmarkListCursor {
                created_at: item.created_at,
                wallet_bookmark_id: item.wallet_bookmark_id,
            })
        } else {
            None
        };

        Ok(WalletBookmarkList {
            owner,
            items,
            next_cursor,
        })
    }
}
