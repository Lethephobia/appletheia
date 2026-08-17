use appletheia::application::read_model::pagination::{CursorPage, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    MaterializedUserStatus, PublicUserList, PublicUserListCriteria, PublicUserListCursor,
    PublicUserListItemPart, PublicUserListReader, PublicUserListReaderError, PublicUserListSortKey,
    UserFragment,
};
use banking_shared_kernel_application::read_model::SearchTerm;
use sqlx::query_builder::Separated;
use sqlx::{Postgres, QueryBuilder};

use super::super::super::projection::PgUserFragmentRow;

/// PostgreSQL-backed public user list reader.
pub struct PgPublicUserListReader;

impl PgPublicUserListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: MaterializedUserStatus) -> &'static str {
        match status {
            MaterializedUserStatus::Active => "active",
            MaterializedUserStatus::Inactive => "inactive",
        }
    }

    fn push_status_in(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        status_in: &[MaterializedUserStatus],
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
            .push("status = ANY(")
            .push_bind_unseparated(status_names)
            .push_unseparated(")");
    }

    /// Pushes terms already normalized by `SearchTerm`.
    ///
    /// The terms are bound verbatim so that this query and the watch matcher share one
    /// normalization; re-normalizing in SQL would reintroduce locale-dependent `lower()`
    /// and POSIX whitespace semantics that the matcher cannot reproduce.
    fn push_username_contains(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        username_search_terms: &[SearchTerm],
    ) {
        let mut pushed_predicate = false;

        for term in username_search_terms {
            if !pushed_predicate {
                predicates.push("username_search_text IS NOT NULL");
                pushed_predicate = true;
            }

            predicates
                .push("username_search_text LIKE likequery(")
                .push_bind_unseparated(term.as_ref())
                .push_unseparated(")");
        }
    }
}

impl Default for PgPublicUserListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicUserListReader for PgPublicUserListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicUserListCriteria,
        sort: Sort<PublicUserListSortKey>,
        page: CursorPage<PublicUserListCursor>,
    ) -> Result<PublicUserList, PublicUserListReaderError> {
        let query_limit = i64::from(page.limit.value()) + 1;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id AS user_id,
                username,
                display_name,
                bio,
                picture_type,
                picture_object_name,
                picture_external_url,
                status,
                created_at,
                source_event_id,
                updated_event_id
            FROM user_fragments
            "#,
        );

        if criteria.status_in.is_some()
            || !criteria.username_contains.is_empty()
            || page.after.is_some()
        {
            builder.push(" WHERE ");
            let mut predicates = builder.separated(" AND ");

            if let Some(status_in) = criteria.status_in.as_deref() {
                Self::push_status_in(&mut predicates, status_in);
            }

            Self::push_username_contains(&mut predicates, &criteria.username_contains);

            if let Some(cursor) = page.after {
                match (sort.key, sort.direction) {
                    (PublicUserListSortKey::CreatedAt, SortDirection::Asc) => {
                        predicates
                            .push("(created_at, id) > (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.user_id.value())
                            .push_unseparated(")");
                    }
                    (PublicUserListSortKey::CreatedAt, SortDirection::Desc) => {
                        predicates
                            .push("(created_at, id) < (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.user_id.value())
                            .push_unseparated(")");
                    }
                    (PublicUserListSortKey::UserId, SortDirection::Asc) => {
                        predicates
                            .push("id > ")
                            .push_bind_unseparated(cursor.user_id.value());
                    }
                    (PublicUserListSortKey::UserId, SortDirection::Desc) => {
                        predicates
                            .push("id < ")
                            .push_bind_unseparated(cursor.user_id.value());
                    }
                }
            }
        }

        match (sort.key, sort.direction) {
            (PublicUserListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY created_at ASC, id ASC");
            }
            (PublicUserListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY created_at DESC, id DESC");
            }
            (PublicUserListSortKey::UserId, SortDirection::Asc) => {
                builder.push(" ORDER BY id ASC");
            }
            (PublicUserListSortKey::UserId, SortDirection::Desc) => {
                builder.push(" ORDER BY id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgUserFragmentRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| PublicUserListReaderError::Persistence(Box::new(error)))?;

        let page_limit = page.limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(UserFragment::try_from)
            .map(|fragment_result| fragment_result.map(PublicUserListItemPart::from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| PublicUserListReaderError::Persistence(Box::new(error)))?;
        let next_cursor = if has_next {
            items.last().map(|item| sort.key.cursor_for_item(item))
        } else {
            None
        };

        Ok(PublicUserList { items, next_cursor })
    }
}

#[cfg(test)]
mod tests {
    use banking_iam_application::{MaterializedUserStatus, PublicUserListCriteria};
    use banking_shared_kernel_application::read_model::SearchTerm;
    use sqlx::{Postgres, QueryBuilder};

    use super::PgPublicUserListReader;

    fn search_term(value: &str) -> SearchTerm {
        SearchTerm::try_from(value).expect("search term should be valid")
    }

    #[test]
    fn username_contains_adds_one_and_predicate_per_term() {
        let criteria = PublicUserListCriteria {
            username_contains: vec![search_term(" Alice "), search_term("bob_smith")],
            ..PublicUserListCriteria::default()
        };
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");
        PgPublicUserListReader::push_status_in(&mut predicates, &[MaterializedUserStatus::Active]);

        PgPublicUserListReader::push_username_contains(
            &mut predicates,
            &criteria.username_contains,
        );

        let sql = builder.sql();
        let sql_text = sql.as_str();
        assert!(sql_text.contains("status = ANY($1) AND username_search_text IS NOT NULL"));
        assert_eq!(
            sql_text.matches(" AND username_search_text LIKE ").count(),
            2
        );
        assert!(sql_text.contains("likequery($2)"));
        assert!(sql_text.contains("likequery($3)"));
    }

    #[test]
    fn empty_username_contains_adds_no_predicate() {
        let criteria = PublicUserListCriteria::default();
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");
        PgPublicUserListReader::push_status_in(&mut predicates, &[MaterializedUserStatus::Active]);

        PgPublicUserListReader::push_username_contains(
            &mut predicates,
            &criteria.username_contains,
        );

        assert_eq!(builder.sql(), "SELECT 1 WHERE status = ANY($1)");
    }

    #[test]
    fn empty_status_in_matches_no_items() {
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");

        PgPublicUserListReader::push_status_in(&mut predicates, &[]);

        assert_eq!(builder.sql(), "SELECT 1 WHERE FALSE");
    }
}
