use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    OwnedAccountList, OwnedAccountListCriteria, OwnedAccountListCursor, OwnedAccountListItem,
    OwnedAccountListItemStatus, OwnedAccountListOwner, OwnedAccountListReader,
    OwnedAccountListReaderError, OwnedAccountListSortKey,
};
use banking_ledger_domain::account::AccountOwner;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use super::pg_owned_account_list_item_row::PgOwnedAccountListItemRow;
use super::pg_owned_account_list_owner_row::PgOwnedAccountListOwnerRow;

/// PostgreSQL-backed owned account list reader.
pub struct PgOwnedAccountListReader;

impl PgOwnedAccountListReader {
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

    fn status_name(status: OwnedAccountListItemStatus) -> &'static str {
        match status {
            OwnedAccountListItemStatus::Active => "active",
            OwnedAccountListItemStatus::Frozen => "frozen",
        }
    }

    async fn read_owner(
        uow: &mut PgUnitOfWork,
        owner_type: &'static str,
        owner_id: Uuid,
    ) -> Result<OwnedAccountListOwner, OwnedAccountListReaderError> {
        let row = sqlx::query_as::<_, PgOwnedAccountListOwnerRow>(
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
            LEFT JOIN owned_account_list_owner_users u
                   ON owner_ref.owner_type = 'user'
                  AND u.id = owner_ref.owner_id
            LEFT JOIN owned_account_list_owner_organizations o
                   ON owner_ref.owner_type = 'organization'
                  AND o.id = owner_ref.owner_id
            "#,
        )
        .bind(owner_type)
        .bind(owner_id)
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountListReaderError::Persistence(Box::new(e)))?;

        OwnedAccountListOwner::try_from(row)
            .map_err(|e| OwnedAccountListReaderError::Persistence(Box::new(e)))
    }
}

impl Default for PgOwnedAccountListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountListReader for PgOwnedAccountListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        owner: AccountOwner,
        criteria: OwnedAccountListCriteria,
        cursor_options: Option<CursorOptions<OwnedAccountListSortKey, OwnedAccountListCursor>>,
        page_size: PageSize,
    ) -> Result<OwnedAccountList, OwnedAccountListReaderError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let owner = Self::read_owner(uow, owner_type, owner_id).await?;
        let limit = i64::from(page_size.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                a.id AS account_id,
                a.name,
                a.currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals,
                c.mint_account_address AS currency_mint_account_address,
                c.source_event_id AS currency_source_event_id,
                c.updated_event_id AS currency_updated_event_id,
                a.balance::text AS balance,
                a.reserved_balance::text AS reserved_balance,
                a.status,
                a.created_at,
                a.source_event_id,
                a.updated_event_id
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
            .unwrap_or(OwnedAccountListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (OwnedAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (a.created_at, a.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (OwnedAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (a.created_at, a.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (OwnedAccountListSortKey::AccountId, SortDirection::Asc) => {
                    builder
                        .push(" AND a.id > ")
                        .push_bind(cursor.account_id.value());
                }
                (OwnedAccountListSortKey::AccountId, SortDirection::Desc) => {
                    builder
                        .push(" AND a.id < ")
                        .push_bind(cursor.account_id.value());
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
            (OwnedAccountListSortKey::AccountId, SortDirection::Asc) => {
                builder.push(" ORDER BY a.id ASC");
            }
            (OwnedAccountListSortKey::AccountId, SortDirection::Desc) => {
                builder.push(" ORDER BY a.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgOwnedAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountListReaderError::Persistence(Box::new(e)))?;

        let limit = page_size.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(OwnedAccountListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| OwnedAccountListReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| OwnedAccountListCursor {
                created_at: item.created_at,
                account_id: item.account_id,
            })
        } else {
            None
        };

        Ok(OwnedAccountList {
            owner,
            items,
            next_cursor,
        })
    }
}
