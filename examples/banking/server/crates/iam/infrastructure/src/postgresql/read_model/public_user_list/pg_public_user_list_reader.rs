use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    PublicUserList, PublicUserListCriteria, PublicUserListCursor, PublicUserListItem,
    PublicUserListReader, PublicUserListReaderError, PublicUserListSortKey,
};
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::query_builder::Separated;
use sqlx::{Postgres, QueryBuilder};

use super::pg_public_user_list_item_row::PgPublicUserListItemRow;

/// PostgreSQL-backed public user list reader.
pub struct PgPublicUserListReader;

impl PgPublicUserListReader {
    pub fn new() -> Self {
        Self
    }

    fn push_username_contains(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        username_contains: &[String],
    ) {
        let mut pushed_predicate = false;

        for contains in username_contains
            .iter()
            .filter(|contains| contains.chars().any(|character| !character.is_whitespace()))
        {
            if !pushed_predicate {
                predicates.push("username_search_text IS NOT NULL");
                pushed_predicate = true;
            }

            predicates
                .push("username_search_text LIKE likequery(regexp_replace(lower(")
                .push_bind_unseparated(contains.as_str())
                .push_unseparated("), '[[:space:]]+', '', 'g'))");
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
        cursor_options: Option<CursorOptions<PublicUserListSortKey, PublicUserListCursor>>,
        limit: PageSize,
    ) -> Result<PublicUserList, PublicUserListReaderError> {
        let query_limit = i64::from(limit.value()) + 1;
        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(PublicUserListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);
        let cursor = cursor_options.and_then(|options| options.cursor);

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id AS user_id,
                username,
                display_name,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            FROM public_user_list_items
            "#,
        );

        builder.push(" WHERE ");
        let mut predicates = builder.separated(" AND ");
        predicates.push("status = 'active'");

        Self::push_username_contains(&mut predicates, &criteria.username_contains);

        if let Some(cursor) = cursor {
            match (sort_key, sort_direction) {
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

        match (sort_key, sort_direction) {
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
            .build_query_as::<PgPublicUserListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| PublicUserListReaderError::Persistence(Box::new(error)))?;

        let page_limit = limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(PublicUserListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| PublicUserListReaderError::Persistence(Box::new(error)))?;
        let next_cursor = if has_next {
            items.last().map(|item| PublicUserListCursor {
                created_at: item.created_at,
                user_id: item.user_id,
            })
        } else {
            None
        };

        Ok(PublicUserList { items, next_cursor })
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Postgres, QueryBuilder};

    use super::PgPublicUserListReader;

    #[test]
    fn username_contains_adds_one_and_predicate_per_non_blank_value() {
        let username_contains = vec![
            " Alice ".to_owned(),
            "bob_smith".to_owned(),
            "   ".to_owned(),
        ];
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");
        predicates.push("status = 'active'");

        PgPublicUserListReader::push_username_contains(&mut predicates, &username_contains);

        let sql = builder.sql();
        let sql_text = sql.as_str();
        assert!(sql_text.contains("status = 'active' AND username_search_text IS NOT NULL"));
        assert_eq!(
            sql_text.matches(" AND username_search_text LIKE ").count(),
            2
        );
        assert!(sql_text.contains("lower($1)"));
        assert!(sql_text.contains("lower($2)"));
    }

    #[test]
    fn empty_username_contains_adds_no_predicate() {
        let username_contains = vec!["   ".to_owned()];
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");
        predicates.push("status = 'active'");

        PgPublicUserListReader::push_username_contains(&mut predicates, &username_contains);

        assert_eq!(builder.sql(), "SELECT 1 WHERE status = 'active'");
    }
}
