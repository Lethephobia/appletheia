use appletheia::application::read_model::pagination::{CursorWindow, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    AccountTransactionStatus, OwnedAccountTransactionList, OwnedAccountTransactionListCriteria,
    OwnedAccountTransactionListCursor, OwnedAccountTransactionListItem,
    OwnedAccountTransactionListOwner, OwnedAccountTransactionListReader,
    OwnedAccountTransactionListReaderError, OwnedAccountTransactionListSortKey,
};
use banking_ledger_domain::account::AccountOwner;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use super::pg_owned_account_transaction_list_item_row::PgOwnedAccountTransactionListItemRow;
use super::pg_owned_account_transaction_list_owner_row::PgOwnedAccountTransactionListOwnerRow;

/// PostgreSQL-backed owned account transaction list reader.
pub struct PgOwnedAccountTransactionListReader;

impl PgOwnedAccountTransactionListReader {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: AccountOwner) -> (&'static str, Uuid) {
        match owner {
            AccountOwner::User(user_id) => ("user", user_id.value()),
            AccountOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: AccountTransactionStatus) -> &'static str {
        match status {
            AccountTransactionStatus::Pending => "pending",
            AccountTransactionStatus::Completed => "completed",
            AccountTransactionStatus::Failed => "failed",
            AccountTransactionStatus::RequiresReview => "requires_review",
        }
    }

    fn push_status_in(
        builder: &mut QueryBuilder<Postgres>,
        status_in: &[AccountTransactionStatus],
    ) {
        if status_in.is_empty() {
            builder.push(" AND FALSE");
            return;
        }

        builder.push(" AND i.status IN (");
        let mut statuses = builder.separated(", ");
        for status in status_in {
            statuses.push_bind_unseparated(Self::status_name(*status));
        }
        statuses.push_unseparated(")");
    }

    async fn read_owner(
        uow: &mut PgUnitOfWork,
        owner_type: &'static str,
        owner_id: Uuid,
    ) -> Result<OwnedAccountTransactionListOwner, OwnedAccountTransactionListReaderError> {
        let row = sqlx::query_as::<_, PgOwnedAccountTransactionListOwnerRow>(
            r#"
            WITH owner_ref AS (
                SELECT $1::text AS owner_type, $2::uuid AS owner_id
            )
            SELECT
                owner_ref.owner_type,
                owner_ref.owner_id,
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
                COALESCE(u.source_event_id, o.source_event_id) AS source_event_id,
                COALESCE(u.updated_event_id, o.updated_event_id) AS updated_event_id
            FROM owner_ref
            LEFT JOIN user_fragments u
                   ON owner_ref.owner_type = 'user'
                  AND u.id = owner_ref.owner_id
            LEFT JOIN organization_fragments o
                   ON owner_ref.owner_type = 'organization'
                  AND o.id = owner_ref.owner_id
            "#,
        )
        .bind(owner_type)
        .bind(owner_id)
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListReaderError::Persistence(Box::new(e)))?;

        OwnedAccountTransactionListOwner::try_from(row)
            .map_err(|e| OwnedAccountTransactionListReaderError::Persistence(Box::new(e)))
    }
}

impl Default for PgOwnedAccountTransactionListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountTransactionListReader for PgOwnedAccountTransactionListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountTransactionListCriteria,
        sort: Sort<OwnedAccountTransactionListSortKey>,
        page: CursorWindow<OwnedAccountTransactionListCursor>,
    ) -> Result<OwnedAccountTransactionList, OwnedAccountTransactionListReaderError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let owner = Self::read_owner(uow, owner_type, owner_id).await?;
        let query_limit = i64::from(page.limit().value()) + 1;
        let query_direction = page.query_direction(sort.direction);

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id AS transaction_id,
                i.transfer_id,
                i.account_id,
                i.counterparty_account_id,
                ca.owner_type AS counterparty_owner_type,
                ca.owner_id AS counterparty_owner_id,
                cu.username AS counterparty_owner_user_username,
                cu.display_name AS counterparty_owner_user_display_name,
                cu.picture_type AS counterparty_owner_user_picture_type,
                cu.picture_object_name AS counterparty_owner_user_picture_object_name,
                cu.picture_external_url AS counterparty_owner_user_picture_external_url,
                co.handle AS counterparty_owner_organization_handle,
                co.display_name AS counterparty_owner_organization_display_name,
                co.picture_type AS counterparty_owner_organization_picture_type,
                co.picture_object_name AS counterparty_owner_organization_picture_object_name,
                co.picture_external_url AS counterparty_owner_organization_picture_external_url,
                COALESCE(cu.source_event_id, co.source_event_id) AS counterparty_owner_source_event_id,
                COALESCE(cu.updated_event_id, co.updated_event_id) AS counterparty_owner_updated_event_id,
                ca.source_event_id AS counterparty_account_source_event_id,
                ca.updated_event_id AS counterparty_account_updated_event_id,
                c.id AS currency_id,
                c.code AS currency_code,
                c.decimals AS currency_decimals,
                c.source_event_id AS currency_source_event_id,
                c.updated_event_id AS currency_updated_event_id,
                i.chain_network,
                i.token_address,
                i.onchain_transaction_id,
                i.amount::text AS amount,
                i.note,
                i.direction,
                i.kind,
                i.status,
                i.occurred_at,
                i.created_at,
                i.source_event_id,
                i.updated_event_id
            FROM account_transaction_fragments i
            INNER JOIN account_fragments a ON a.id = i.account_id
            INNER JOIN currency_fragments c ON c.id = a.currency_id
            LEFT JOIN account_fragments ca ON ca.id = i.counterparty_account_id
            LEFT JOIN user_fragments cu
                ON ca.owner_type = 'user' AND cu.id = ca.owner_id
            LEFT JOIN organization_fragments co
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

        if let Some(currency_code) = criteria.currency_code {
            builder
                .push(" AND c.code = ")
                .push_bind(currency_code.value().to_owned());
        }

        if let Some(status_in) = criteria.status_in.as_deref() {
            Self::push_status_in(&mut builder, status_in);
        }

        if let Some(cursor) = page.boundary().copied() {
            match (sort.key, query_direction) {
                (OwnedAccountTransactionListSortKey::OccurredAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.occurred_at, i.id) > (")
                        .push_bind(cursor.occurred_at.value())
                        .push(", ")
                        .push_bind(cursor.transaction_id.value())
                        .push(")");
                }
                (OwnedAccountTransactionListSortKey::OccurredAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.occurred_at, i.id) < (")
                        .push_bind(cursor.occurred_at.value())
                        .push(", ")
                        .push_bind(cursor.transaction_id.value())
                        .push(")");
                }
                (OwnedAccountTransactionListSortKey::TransactionId, SortDirection::Asc) => {
                    builder
                        .push(" AND i.id > ")
                        .push_bind(cursor.transaction_id.value());
                }
                (OwnedAccountTransactionListSortKey::TransactionId, SortDirection::Desc) => {
                    builder
                        .push(" AND i.id < ")
                        .push_bind(cursor.transaction_id.value());
                }
            }
        }

        match (sort.key, query_direction) {
            (OwnedAccountTransactionListSortKey::OccurredAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.occurred_at ASC, i.id ASC");
            }
            (OwnedAccountTransactionListSortKey::OccurredAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.occurred_at DESC, i.id DESC");
            }
            (OwnedAccountTransactionListSortKey::TransactionId, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (OwnedAccountTransactionListSortKey::TransactionId, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgOwnedAccountTransactionListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountTransactionListReaderError::Persistence(Box::new(e)))?;

        let page_limit = page.limit().value() as usize;
        let has_more = rows.len() > page_limit;
        let mut items = rows
            .into_iter()
            .take(page_limit)
            .map(OwnedAccountTransactionListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| OwnedAccountTransactionListReaderError::Persistence(Box::new(e)))?;
        if page.is_backward() {
            items.reverse();
        }
        let start_cursor = items.first().map(|item| OwnedAccountTransactionListCursor {
            occurred_at: item.occurred_at,
            transaction_id: item.transaction_id,
        });
        let end_cursor = items.last().map(|item| OwnedAccountTransactionListCursor {
            occurred_at: item.occurred_at,
            transaction_id: item.transaction_id,
        });
        let (has_previous, has_next) = if page.is_backward() {
            (has_more, !items.is_empty() && page.boundary().is_some())
        } else {
            (!items.is_empty() && page.boundary().is_some(), has_more)
        };

        Ok(OwnedAccountTransactionList {
            owner,
            items,
            start_cursor,
            end_cursor,
            has_previous,
            has_next,
        })
    }
}
