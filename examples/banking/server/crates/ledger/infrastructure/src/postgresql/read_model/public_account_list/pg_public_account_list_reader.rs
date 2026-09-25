use appletheia::application::read_model::pagination::{CursorWindow, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    MaterializedAccountStatus, PublicAccountList, PublicAccountListCriteria,
    PublicAccountListCursor, PublicAccountListItem, PublicAccountListReader,
    PublicAccountListReaderError, PublicAccountListSortKey,
};
use banking_ledger_domain::account::AccountOwner;
use sqlx::query_builder::Separated;
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

    fn status_name(status: MaterializedAccountStatus) -> &'static str {
        match status {
            MaterializedAccountStatus::Active => "active",
            MaterializedAccountStatus::Frozen => "frozen",
        }
    }

    fn push_status_in(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        status_in: &[MaterializedAccountStatus],
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
            .push("a.status = ANY(")
            .push_bind_unseparated(status_names)
            .push_unseparated(")");
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
        sort: Sort<PublicAccountListSortKey>,
        page: CursorWindow<PublicAccountListCursor>,
    ) -> Result<PublicAccountList, PublicAccountListReaderError> {
        let query_limit = i64::from(page.limit().value()) + 1;
        let query_direction = page.query_direction(sort.direction);

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
                c.code AS currency_code,
                c.decimals AS currency_decimals,
                c.source_event_id AS currency_source_event_id,
                c.updated_event_id AS currency_updated_event_id,
                a.status,
                a.created_at,
                a.source_event_id,
                a.updated_event_id
              FROM account_fragments a
              INNER JOIN currency_fragments c ON c.id = a.currency_id
              LEFT JOIN user_fragments u
                     ON a.owner_type = 'user'
                    AND u.id = a.owner_id
              LEFT JOIN organization_fragments o
                     ON a.owner_type = 'organization'
                    AND o.id = a.owner_id
            "#,
        );

        if criteria.owner.is_some()
            || criteria.currency_code.is_some()
            || criteria.status_in.is_some()
            || page.boundary().is_some()
        {
            builder.push(" WHERE ");
            let mut predicates = builder.separated(" AND ");

            if let Some(owner) = criteria.owner {
                let (owner_type, owner_id) = Self::owner_parts(owner);
                predicates
                    .push("a.owner_type = ")
                    .push_bind_unseparated(owner_type);
                predicates
                    .push("a.owner_id = ")
                    .push_bind_unseparated(owner_id);
            }

            if let Some(currency_code) = criteria.currency_code {
                predicates
                    .push("c.code = ")
                    .push_bind_unseparated(currency_code.value().to_owned());
            }

            if let Some(status_in) = criteria.status_in.as_deref() {
                Self::push_status_in(&mut predicates, status_in);
            }

            if let Some(cursor) = page.boundary().copied() {
                match (sort.key, query_direction) {
                    (PublicAccountListSortKey::CreatedAt, SortDirection::Asc) => {
                        predicates
                            .push("(a.created_at, a.id) > (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.account_id.value())
                            .push_unseparated(")");
                    }
                    (PublicAccountListSortKey::CreatedAt, SortDirection::Desc) => {
                        predicates
                            .push("(a.created_at, a.id) < (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.account_id.value())
                            .push_unseparated(")");
                    }
                    (PublicAccountListSortKey::AccountId, SortDirection::Asc) => {
                        predicates
                            .push("a.id > ")
                            .push_bind_unseparated(cursor.account_id.value());
                    }
                    (PublicAccountListSortKey::AccountId, SortDirection::Desc) => {
                        predicates
                            .push("a.id < ")
                            .push_bind_unseparated(cursor.account_id.value());
                    }
                }
            }
        }

        match (sort.key, query_direction) {
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

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgPublicAccountListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| PublicAccountListReaderError::Persistence(Box::new(e)))?;

        let page_limit = page.limit().value() as usize;
        let has_more = rows.len() > page_limit;
        let mut items = rows
            .into_iter()
            .take(page_limit)
            .map(PublicAccountListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| PublicAccountListReaderError::Persistence(Box::new(e)))?;
        if page.is_backward() {
            items.reverse();
        }
        let start_cursor = items.first().map(|item| PublicAccountListCursor {
            created_at: item.created_at,
            account_id: item.account_id,
        });
        let end_cursor = items.last().map(|item| PublicAccountListCursor {
            created_at: item.created_at,
            account_id: item.account_id,
        });
        let (has_previous, has_next) = if page.is_backward() {
            (has_more, !items.is_empty() && page.boundary().is_some())
        } else {
            (!items.is_empty() && page.boundary().is_some(), has_more)
        };

        Ok(PublicAccountList {
            items,
            start_cursor,
            end_cursor,
            has_previous,
            has_next,
        })
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_application::MaterializedAccountStatus;
    use sqlx::{Postgres, QueryBuilder};

    use super::PgPublicAccountListReader;

    #[test]
    fn status_in_adds_one_array_predicate() {
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");

        PgPublicAccountListReader::push_status_in(
            &mut predicates,
            &[
                MaterializedAccountStatus::Active,
                MaterializedAccountStatus::Frozen,
            ],
        );

        assert_eq!(builder.sql(), "SELECT 1 WHERE a.status = ANY($1)");
    }

    #[test]
    fn empty_status_in_matches_no_items() {
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");

        PgPublicAccountListReader::push_status_in(&mut predicates, &[]);

        assert_eq!(builder.sql(), "SELECT 1 WHERE FALSE");
    }
}
