use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CursorOptions, OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCriteria,
    OwnedAccountTransactionListItemCursor, OwnedAccountTransactionListItemReader,
    OwnedAccountTransactionListItemReaderError, OwnedAccountTransactionListItemSortKey,
    OwnedAccountTransactionListItemStatus, Page, PageLimit, SortDirection,
};
use banking_ledger_domain::account::AccountOwner;
use sqlx::{Postgres, QueryBuilder};

use super::pg_owned_account_transaction_list_item_row::PgOwnedAccountTransactionListItemRow;

/// PostgreSQL-backed owned account transaction list item reader.
pub struct PgOwnedAccountTransactionListItemReader;

impl PgOwnedAccountTransactionListItemReader {
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

    fn status_name(status: OwnedAccountTransactionListItemStatus) -> &'static str {
        match status {
            OwnedAccountTransactionListItemStatus::Pending => "pending",
            OwnedAccountTransactionListItemStatus::Completed => "completed",
            OwnedAccountTransactionListItemStatus::Failed => "failed",
            OwnedAccountTransactionListItemStatus::RequiresReview => "requires_review",
        }
    }
}

impl Default for PgOwnedAccountTransactionListItemReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountTransactionListItemReader for PgOwnedAccountTransactionListItemReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountTransactionListItemCriteria,
        cursor_options: Option<
            CursorOptions<
                OwnedAccountTransactionListItemSortKey,
                OwnedAccountTransactionListItemCursor,
            >,
        >,
        page_limit: PageLimit,
    ) -> Result<
        Page<OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCursor>,
        OwnedAccountTransactionListItemReaderError,
    > {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let limit = i64::from(page_limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id,
                i.transfer_id,
                i.owner_type,
                i.owner_id,
                i.account_id,
                i.counterparty_account_id,
                ca.owner_type AS counterparty_owner_type,
                ca.owner_id AS counterparty_owner_id,
                cu.username AS counterparty_owner_user_username,
                cu.display_name AS counterparty_owner_user_display_name,
                cu.picture AS counterparty_owner_user_picture,
                co.handle AS counterparty_owner_organization_handle,
                co.display_name AS counterparty_owner_organization_display_name,
                co.picture AS counterparty_owner_organization_picture,
                i.currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals,
                i.amount::text AS amount,
                i.direction,
                i.kind,
                i.status,
                i.occurred_at,
                i.created_at
            FROM owned_account_transaction_list_items i
            INNER JOIN owned_account_transaction_list_item_currencies c ON c.id = i.currency_id
            LEFT JOIN owned_account_list_items ca ON ca.id = i.counterparty_account_id
            LEFT JOIN owned_account_transaction_list_item_owner_users cu
                ON ca.owner_type = 'user' AND cu.id = ca.owner_id
            LEFT JOIN owned_account_transaction_list_item_owner_organizations co
                ON ca.owner_type = 'organization' AND co.id = ca.owner_id
            WHERE i.owner_type =
            "#,
        );

        builder
            .push_bind(owner_type)
            .push(" AND i.owner_id = ")
            .push_bind(owner_id);

        if let Some(account_id) = criteria.account_id {
            builder
                .push(" AND i.account_id = ")
                .push_bind(account_id.value());
        }

        if let Some(currency_id) = criteria.currency_id {
            builder
                .push(" AND i.currency_id = ")
                .push_bind(currency_id.value());
        }

        if let Some(status) = criteria.status {
            builder
                .push(" AND i.status = ")
                .push_bind(Self::status_name(status));
        }

        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OwnedAccountTransactionListItemSortKey::OccurredAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (OwnedAccountTransactionListItemSortKey::OccurredAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.occurred_at, i.id) > (")
                        .push_bind(cursor.occurred_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountTransactionListItemSortKey::OccurredAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.occurred_at, i.id) < (")
                        .push_bind(cursor.occurred_at.value())
                        .push(", ")
                        .push_bind(cursor.id.value())
                        .push(")");
                }
                (OwnedAccountTransactionListItemSortKey::Id, SortDirection::Asc) => {
                    builder.push(" AND i.id > ").push_bind(cursor.id.value());
                }
                (OwnedAccountTransactionListItemSortKey::Id, SortDirection::Desc) => {
                    builder.push(" AND i.id < ").push_bind(cursor.id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (OwnedAccountTransactionListItemSortKey::OccurredAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.occurred_at ASC, i.id ASC");
            }
            (OwnedAccountTransactionListItemSortKey::OccurredAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.occurred_at DESC, i.id DESC");
            }
            (OwnedAccountTransactionListItemSortKey::Id, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (OwnedAccountTransactionListItemSortKey::Id, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgOwnedAccountTransactionListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountTransactionListItemReaderError::Persistence(Box::new(e)))?;

        let limit = page_limit.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(OwnedAccountTransactionListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| OwnedAccountTransactionListItemReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items
                .last()
                .map(|item| OwnedAccountTransactionListItemCursor {
                    occurred_at: item.occurred_at,
                    id: item.id,
                })
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }
}
