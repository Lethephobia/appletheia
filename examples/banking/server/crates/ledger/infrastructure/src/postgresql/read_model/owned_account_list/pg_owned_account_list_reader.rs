use appletheia::application::read_model::pagination::{CursorPage, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    MaterializedAccountStatus, OwnedAccountList, OwnedAccountListCriteria, OwnedAccountListCursor,
    OwnedAccountListItemPart, OwnedAccountListOwner, OwnedAccountListReader,
    OwnedAccountListReaderError, OwnedAccountListSortKey,
};
use banking_ledger_domain::account::AccountOwner;
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

    fn status_name(status: MaterializedAccountStatus) -> &'static str {
        match status {
            MaterializedAccountStatus::Active => "active",
            MaterializedAccountStatus::Frozen => "frozen",
        }
    }

    fn push_status_in(
        builder: &mut QueryBuilder<Postgres>,
        status_in: &[MaterializedAccountStatus],
    ) {
        if status_in.is_empty() {
            builder.push(" AND FALSE");
            return;
        }

        builder.push(" AND a.status IN (");
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
        sort: Sort<OwnedAccountListSortKey>,
        page: CursorPage<OwnedAccountListCursor>,
    ) -> Result<OwnedAccountList, OwnedAccountListReaderError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let owner = Self::read_owner(uow, owner_type, owner_id).await?;
        let query_limit = i64::from(page.limit.value()) + 1;

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
            FROM account_fragments a
            INNER JOIN currency_fragments c ON c.id = a.currency_id
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

        if let Some(status_in) = criteria.status_in.as_deref() {
            Self::push_status_in(&mut builder, status_in);
        }

        if let Some(cursor) = page.after {
            match (sort.key, sort.direction) {
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

        match (sort.key, sort.direction) {
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

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgOwnedAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| OwnedAccountListReaderError::Persistence(Box::new(e)))?;

        let page_limit = page.limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(OwnedAccountListItemPart::try_from)
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
