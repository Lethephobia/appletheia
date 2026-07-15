use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    PublicAccountList, PublicAccountListCriteria, PublicAccountListCursor, PublicAccountListItem,
    PublicAccountListReader, PublicAccountListReaderError, PublicAccountListSortKey,
};
use banking_ledger_domain::account::AccountOwner;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use super::pg_public_account_list_item_row::PgPublicAccountListItemRow;

/// PostgreSQL-backed public account list reader.
pub struct PgPublicAccountListReader;

impl PgPublicAccountListReader {
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
}

impl Default for PgPublicAccountListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicAccountListReader for PgPublicAccountListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicAccountListCriteria,
        cursor_options: Option<CursorOptions<PublicAccountListSortKey, PublicAccountListCursor>>,
        page_size: PageSize,
    ) -> Result<PublicAccountList, PublicAccountListReaderError> {
        let limit = i64::from(page_size.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                a.id AS account_id,
                a.owner_type,
                a.owner_id,
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
                c.id AS currency_id,
                c.symbol AS currency_symbol,
                c.name AS currency_name,
                c.decimals AS currency_decimals,
                c.mint_account_address AS currency_mint_account_address,
                c.source_event_id AS currency_source_event_id,
                c.updated_event_id AS currency_updated_event_id,
                a.created_at,
                a.source_event_id,
                a.updated_event_id
              FROM public_account_list_items a
              INNER JOIN public_account_list_item_currencies c
                      ON c.id = a.currency_id
              LEFT JOIN public_account_list_item_owner_users u
                     ON a.owner_type = 'user'
                    AND u.id = a.owner_id
              LEFT JOIN public_account_list_item_owner_organizations o
                     ON a.owner_type = 'organization'
                    AND o.id = a.owner_id
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
            .unwrap_or(PublicAccountListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);

        if let Some(cursor) = cursor_options.and_then(|options| options.cursor) {
            match (sort_key, sort_direction) {
                (PublicAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (a.created_at, a.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (PublicAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (a.created_at, a.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.account_id.value())
                        .push(")");
                }
                (PublicAccountListSortKey::AccountId, SortDirection::Asc) => {
                    builder
                        .push(" AND a.id > ")
                        .push_bind(cursor.account_id.value());
                }
                (PublicAccountListSortKey::AccountId, SortDirection::Desc) => {
                    builder
                        .push(" AND a.id < ")
                        .push_bind(cursor.account_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (PublicAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY a.created_at ASC, a.id ASC");
            }
            (PublicAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY a.created_at DESC, a.id DESC");
            }
            (PublicAccountListSortKey::AccountId, SortDirection::Asc) => {
                builder.push(" ORDER BY a.id ASC");
            }
            (PublicAccountListSortKey::AccountId, SortDirection::Desc) => {
                builder.push(" ORDER BY a.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(limit);

        let rows = builder
            .build_query_as::<PgPublicAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| PublicAccountListReaderError::Persistence(Box::new(e)))?;

        let limit = page_size.value() as usize;
        let has_next = rows.len() > limit;
        let items = rows
            .into_iter()
            .take(limit)
            .map(PublicAccountListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| PublicAccountListReaderError::Persistence(Box::new(e)))?;
        let next_cursor = if has_next {
            items.last().map(|item| PublicAccountListCursor {
                created_at: item.created_at,
                account_id: item.account_id,
            })
        } else {
            None
        };

        Ok(PublicAccountList { items, next_cursor })
    }
}
