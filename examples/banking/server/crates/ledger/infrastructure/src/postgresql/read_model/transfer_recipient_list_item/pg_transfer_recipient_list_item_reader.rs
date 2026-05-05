use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CursorOptions, Page, PageLimit, SortDirection, TransferRecipientListItem,
    TransferRecipientListItemCursor, TransferRecipientListItemReader,
    TransferRecipientListItemReaderError, TransferRecipientListItemSortKey,
};
use banking_ledger_domain::currency::CurrencyId;
use sqlx::{Postgres, QueryBuilder};

use super::pg_transfer_recipient_list_item_row::PgTransferRecipientListItemRow;

/// PostgreSQL-backed transfer recipient list item reader.
pub struct PgTransferRecipientListItemReader;

impl PgTransferRecipientListItemReader {
    pub fn new() -> Self {
        Self
    }

    fn keyword_pattern(keyword: Option<String>) -> Option<String> {
        keyword
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty())
            .map(|value| {
                let mut pattern = String::with_capacity(value.len() + 1);

                for character in value.chars() {
                    if matches!(character, '%' | '_' | '\\') {
                        pattern.push('\\');
                    }

                    pattern.push(character);
                }

                pattern.push('%');
                pattern
            })
    }
}

impl Default for PgTransferRecipientListItemReader {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferRecipientListItemReader for PgTransferRecipientListItemReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        keyword: Option<String>,
        currency_id: Option<CurrencyId>,
        cursor_options: Option<
            CursorOptions<TransferRecipientListItemSortKey, TransferRecipientListItemCursor>,
        >,
        page_limit: PageLimit,
    ) -> Result<
        Page<TransferRecipientListItem, TransferRecipientListItemCursor>,
        TransferRecipientListItemReaderError,
    > {
        let limit = i64::from(page_limit.value()) + 1;
        let keyword_pattern = Self::keyword_pattern(keyword);

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(TransferRecipientListItemSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            WITH page_users AS (
                SELECT u.id, u.username, u.display_name, u.picture, u.created_at
                  FROM transfer_recipient_list_item_users u
                 WHERE u.status = 'active'
                   AND EXISTS (
                       SELECT 1
                         FROM transfer_recipient_list_item_accounts ea
                         INNER JOIN transfer_recipient_list_item_currencies ec
                                 ON ec.id = ea.currency_id
                        WHERE ea.owner_type = 'user'
                          AND ea.owner_id = u.id
                          AND ea.status = 'active'
            "#,
        );

        if let Some(currency_id) = currency_id {
            builder
                .push(" AND ea.currency_id = ")
                .push_bind(currency_id.value());
        }

        builder.push(")");

        if let Some(keyword_pattern) = keyword_pattern.as_ref() {
            builder
                .push(" AND (lower(u.username) LIKE ")
                .push_bind(keyword_pattern.as_str())
                .push(r" ESCAPE '\' OR lower(u.display_name) LIKE ")
                .push_bind(keyword_pattern.as_str())
                .push(r" ESCAPE '\')");
        }

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (u.created_at, u.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.user_id.value())
                        .push(")");
                }
                (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (u.created_at, u.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.user_id.value())
                        .push(")");
                }
                (TransferRecipientListItemSortKey::UserId, SortDirection::Asc) => {
                    builder
                        .push(" AND u.id > ")
                        .push_bind(cursor.user_id.value());
                }
                (TransferRecipientListItemSortKey::UserId, SortDirection::Desc) => {
                    builder
                        .push(" AND u.id < ")
                        .push_bind(cursor.user_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY u.created_at ASC, u.id ASC");
            }
            (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY u.created_at DESC, u.id DESC");
            }
            (TransferRecipientListItemSortKey::UserId, SortDirection::Asc) => {
                builder.push(" ORDER BY u.id ASC");
            }
            (TransferRecipientListItemSortKey::UserId, SortDirection::Desc) => {
                builder.push(" ORDER BY u.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit).push(
            r#"
            )
            SELECT
                u.id AS user_id,
                u.username,
                u.display_name,
                u.picture,
                u.created_at AS user_created_at,
                a.id AS account_id,
                c.id AS currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals
              FROM page_users u
              INNER JOIN transfer_recipient_list_item_accounts a
                      ON a.owner_type = 'user'
                     AND a.owner_id = u.id
                     AND a.status = 'active'
              INNER JOIN transfer_recipient_list_item_currencies c
                      ON c.id = a.currency_id
             WHERE TRUE
            "#,
        );

        if let Some(currency_id) = currency_id {
            builder
                .push(" AND a.currency_id = ")
                .push_bind(currency_id.value());
        }

        match (sort_key, sort_direction) {
            (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY u.created_at ASC, u.id ASC, a.created_at DESC, a.id DESC");
            }
            (TransferRecipientListItemSortKey::CreatedAt, SortDirection::Desc) => {
                builder
                    .push(" ORDER BY u.created_at DESC, u.id DESC, a.created_at DESC, a.id DESC");
            }
            (TransferRecipientListItemSortKey::UserId, SortDirection::Asc) => {
                builder.push(" ORDER BY u.id ASC, a.created_at DESC, a.id DESC");
            }
            (TransferRecipientListItemSortKey::UserId, SortDirection::Desc) => {
                builder.push(" ORDER BY u.id DESC, a.created_at DESC, a.id DESC");
            }
        }

        let rows = builder
            .build_query_as::<PgTransferRecipientListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| TransferRecipientListItemReaderError::Persistence(Box::new(e)))?;

        let mut items = Vec::<TransferRecipientListItem>::new();

        for row in rows {
            let user_id = row
                .user_id()
                .map_err(|e| TransferRecipientListItemReaderError::Persistence(Box::new(e)))?;

            if items.last().map(|item| item.user_id) != Some(user_id) {
                items.push(
                    row.list_item().map_err(|e| {
                        TransferRecipientListItemReaderError::Persistence(Box::new(e))
                    })?,
                );
            }

            items
                .last_mut()
                .expect("item was just inserted or already present")
                .accounts
                .push(
                    row.account().map_err(|e| {
                        TransferRecipientListItemReaderError::Persistence(Box::new(e))
                    })?,
                );
        }

        let limit = page_limit.value() as usize;
        let has_next = items.len() > limit;
        let next_cursor = if has_next {
            items
                .get(limit - 1)
                .map(|item| TransferRecipientListItemCursor {
                    created_at: item.created_at,
                    user_id: item.user_id,
                })
        } else {
            None
        };
        items.truncate(limit);

        Ok(Page { items, next_cursor })
    }
}
