use appletheia::application::read_model::pagination::{CursorWindow, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyList, CurrencyListCriteria, CurrencyListCursor, CurrencyListItem, CurrencyListReader,
    CurrencyListReaderError, CurrencyListSortKey, MaterializedCurrencyStatus,
};
use sqlx::query_builder::Separated;
use sqlx::{Postgres, QueryBuilder};

use super::pg_currency_list_item_row::PgCurrencyListItemRow;

/// PostgreSQL-backed currency list reader.
pub struct PgCurrencyListReader;

impl PgCurrencyListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: MaterializedCurrencyStatus) -> &'static str {
        match status {
            MaterializedCurrencyStatus::Provisioning => "provisioning",
            MaterializedCurrencyStatus::Active => "active",
            MaterializedCurrencyStatus::Inactive => "inactive",
            MaterializedCurrencyStatus::ProvisioningFailed => "provisioning_failed",
        }
    }

    fn push_status_in(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        status_in: &[MaterializedCurrencyStatus],
    ) {
        if status_in.is_empty() {
            predicates.push("FALSE");
            return;
        }

        let status_names = status_in
            .iter()
            .map(|status| Self::status_name(*status).to_owned())
            .collect::<Vec<_>>();
        predicates
            .push("i.status = ANY(")
            .push_bind_unseparated(status_names)
            .push_unseparated(")");
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
        sort: Sort<CurrencyListSortKey>,
        page: CursorWindow<CurrencyListCursor>,
    ) -> Result<CurrencyList, CurrencyListReaderError> {
        let query_limit = i64::from(page.limit().value()) + 1;
        let query_direction = page.query_direction(sort.direction);

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
                i.mint_account_address,
                i.supply::text AS supply,
                i.status,
                i.created_at,
                i.source_event_id,
                i.updated_event_id
            FROM currency_fragments i
            LEFT JOIN user_fragments u
                   ON i.owner_type = 'user'
                  AND u.id = i.owner_id
            LEFT JOIN organization_fragments o
                   ON i.owner_type = 'organization'
                  AND o.id = i.owner_id
            "#,
        );

        if criteria.status_in.is_some() || page.boundary().is_some() {
            builder.push(" WHERE ");
            let mut predicates = builder.separated(" AND ");

            if let Some(status_in) = criteria.status_in.as_deref() {
                Self::push_status_in(&mut predicates, status_in);
            }

            if let Some(cursor) = page.boundary().copied() {
                match (sort.key, query_direction) {
                    (CurrencyListSortKey::CreatedAt, SortDirection::Asc) => {
                        predicates
                            .push("(i.created_at, i.id) > (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.currency_id.value())
                            .push_unseparated(")");
                    }
                    (CurrencyListSortKey::CreatedAt, SortDirection::Desc) => {
                        predicates
                            .push("(i.created_at, i.id) < (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.currency_id.value())
                            .push_unseparated(")");
                    }
                    (CurrencyListSortKey::CurrencyId, SortDirection::Asc) => {
                        predicates
                            .push("i.id > ")
                            .push_bind_unseparated(cursor.currency_id.value());
                    }
                    (CurrencyListSortKey::CurrencyId, SortDirection::Desc) => {
                        predicates
                            .push("i.id < ")
                            .push_bind_unseparated(cursor.currency_id.value());
                    }
                }
            }
        }

        match (sort.key, query_direction) {
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

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgCurrencyListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| CurrencyListReaderError::Persistence(Box::new(e)))?;

        let page_limit = page.limit().value() as usize;
        let has_more = rows.len() > page_limit;
        let mut items = rows
            .into_iter()
            .take(page_limit)
            .map(CurrencyListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CurrencyListReaderError::Persistence(Box::new(e)))?;
        if page.is_backward() {
            items.reverse();
        }
        let start_cursor = items.first().map(|item| CurrencyListCursor {
            created_at: item.created_at,
            currency_id: item.currency_id,
        });
        let end_cursor = items.last().map(|item| CurrencyListCursor {
            created_at: item.created_at,
            currency_id: item.currency_id,
        });
        let (has_previous, has_next) = if page.is_backward() {
            (has_more, !items.is_empty() && page.boundary().is_some())
        } else {
            (!items.is_empty() && page.boundary().is_some(), has_more)
        };

        Ok(CurrencyList {
            items,
            start_cursor,
            end_cursor,
            has_previous,
            has_next,
        })
    }
}
