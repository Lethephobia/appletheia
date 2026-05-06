use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CursorOptions, Page, PageLimit, PublicAccountListItem, PublicAccountListItemCriteria,
    PublicAccountListItemCursor, PublicAccountListItemReader, PublicAccountListItemReaderError,
    PublicAccountListItemSortKey, SortDirection,
};
use banking_ledger_domain::account::AccountOwner;
use sqlx::{Postgres, QueryBuilder};

use super::pg_public_account_list_item_row::PgPublicAccountListItemRow;

/// PostgreSQL-backed public account list item reader.
pub struct PgPublicAccountListItemReader;

impl PgPublicAccountListItemReader {
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
}

impl Default for PgPublicAccountListItemReader {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicAccountListItemReader for PgPublicAccountListItemReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicAccountListItemCriteria,
        cursor_options: Option<
            CursorOptions<PublicAccountListItemSortKey, PublicAccountListItemCursor>,
        >,
        page_limit: PageLimit,
    ) -> Result<
        Page<PublicAccountListItem, PublicAccountListItemCursor>,
        PublicAccountListItemReaderError,
    > {
        let limit = i64::from(page_limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                a.id AS account_id,
                a.owner_type,
                a.owner_id,
                c.id AS currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals,
                a.created_at
              FROM public_account_list_item_accounts a
              INNER JOIN public_account_list_item_currencies c
                      ON c.id = a.currency_id
             WHERE a.status = 'active'
            "#,
        );

        if let Some(owner) = criteria.owner {
            let (owner_type, owner_id) = Self::owner_parts(owner);
            builder
                .push(" AND a.owner_type = ")
                .push_bind(owner_type)
                .push(" AND a.owner_id = ")
                .push_bind(owner_id);
        }

        if let Some(currency_id) = criteria.currency_id {
            builder
                .push(" AND a.currency_id = ")
                .push_bind(currency_id.value());
        }

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(PublicAccountListItemSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (PublicAccountListItemSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (a.created_at, a.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (PublicAccountListItemSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (a.created_at, a.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (PublicAccountListItemSortKey::AccountId, SortDirection::Asc) => {
                    builder
                        .push(" AND a.id > ")
                        .push_bind(cursor.account_id.value());
                }
                (PublicAccountListItemSortKey::AccountId, SortDirection::Desc) => {
                    builder
                        .push(" AND a.id < ")
                        .push_bind(cursor.account_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (PublicAccountListItemSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY a.created_at ASC, a.id ASC");
            }
            (PublicAccountListItemSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY a.created_at DESC, a.id DESC");
            }
            (PublicAccountListItemSortKey::AccountId, SortDirection::Asc) => {
                builder.push(" ORDER BY a.id ASC");
            }
            (PublicAccountListItemSortKey::AccountId, SortDirection::Desc) => {
                builder.push(" ORDER BY a.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgPublicAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| PublicAccountListItemReaderError::Persistence(Box::new(e)))?;

        let limit = page_limit.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(PublicAccountListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| PublicAccountListItemReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| PublicAccountListItemCursor {
                created_at: item.created_at,
                account_id: item.account_id,
            })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
