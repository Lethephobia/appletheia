use appletheia::application::read_model::pagination::{CursorPage, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    PublicOrganizationList, PublicOrganizationListCriteria, PublicOrganizationListCursor,
    PublicOrganizationListItemPart, PublicOrganizationListReader,
    PublicOrganizationListReaderError, PublicOrganizationListSortKey,
};
use banking_shared_kernel_application::read_model::SearchTerm;
use sqlx::query_builder::Separated;
use sqlx::{Postgres, QueryBuilder};

use super::pg_public_organization_list_item_row::PgPublicOrganizationListItemRow;

/// PostgreSQL-backed public organization list reader.
pub struct PgPublicOrganizationListReader;

impl PgPublicOrganizationListReader {
    pub fn new() -> Self {
        Self
    }

    fn push_handle_contains(
        predicates: &mut Separated<'_, Postgres, &'static str>,
        handle_contains: &[SearchTerm],
    ) {
        let mut pushed_predicate = false;

        for term in handle_contains {
            if !pushed_predicate {
                predicates.push("handle_search_text IS NOT NULL");
                pushed_predicate = true;
            }

            predicates
                .push("handle_search_text LIKE likequery(")
                .push_bind_unseparated(term.as_ref())
                .push_unseparated(")");
        }
    }
}

impl Default for PgPublicOrganizationListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicOrganizationListReader for PgPublicOrganizationListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        criteria: PublicOrganizationListCriteria,
        sort: Sort<PublicOrganizationListSortKey>,
        page: CursorPage<PublicOrganizationListCursor>,
    ) -> Result<PublicOrganizationList, PublicOrganizationListReaderError> {
        let query_limit = i64::from(page.limit.value()) + 1;
        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id AS organization_id,
                handle,
                display_name,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            FROM organization_fragments
            "#,
        );

        if !criteria.handle_contains.is_empty() || page.after.is_some() {
            builder.push(" WHERE ");
            let mut predicates = builder.separated(" AND ");

            Self::push_handle_contains(&mut predicates, &criteria.handle_contains);

            if let Some(cursor) = page.after {
                match (sort.key, sort.direction) {
                    (PublicOrganizationListSortKey::CreatedAt, SortDirection::Asc) => {
                        predicates
                            .push("(created_at, id) > (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.organization_id.value())
                            .push_unseparated(")");
                    }
                    (PublicOrganizationListSortKey::CreatedAt, SortDirection::Desc) => {
                        predicates
                            .push("(created_at, id) < (")
                            .push_bind_unseparated(cursor.created_at.value())
                            .push_unseparated(", ")
                            .push_bind_unseparated(cursor.organization_id.value())
                            .push_unseparated(")");
                    }
                    (PublicOrganizationListSortKey::OrganizationId, SortDirection::Asc) => {
                        predicates
                            .push("id > ")
                            .push_bind_unseparated(cursor.organization_id.value());
                    }
                    (PublicOrganizationListSortKey::OrganizationId, SortDirection::Desc) => {
                        predicates
                            .push("id < ")
                            .push_bind_unseparated(cursor.organization_id.value());
                    }
                }
            }
        }

        match (sort.key, sort.direction) {
            (PublicOrganizationListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY created_at ASC, id ASC");
            }
            (PublicOrganizationListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY created_at DESC, id DESC");
            }
            (PublicOrganizationListSortKey::OrganizationId, SortDirection::Asc) => {
                builder.push(" ORDER BY id ASC");
            }
            (PublicOrganizationListSortKey::OrganizationId, SortDirection::Desc) => {
                builder.push(" ORDER BY id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgPublicOrganizationListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| PublicOrganizationListReaderError::Persistence(Box::new(error)))?;

        let page_limit = page.limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(PublicOrganizationListItemPart::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| PublicOrganizationListReaderError::Persistence(Box::new(error)))?;
        let next_cursor = if has_next {
            items.last().map(|item| PublicOrganizationListCursor {
                created_at: item.created_at,
                organization_id: item.organization_id,
            })
        } else {
            None
        };

        Ok(PublicOrganizationList { items, next_cursor })
    }
}

#[cfg(test)]
mod tests {
    use banking_shared_kernel_application::read_model::SearchTerm;
    use sqlx::{Postgres, QueryBuilder};

    use super::PgPublicOrganizationListReader;

    fn search_term(value: &str) -> SearchTerm {
        SearchTerm::try_from(value).expect("search term should be valid")
    }

    #[test]
    fn handle_contains_adds_one_and_predicate_per_term() {
        let handle_contains = vec![search_term(" Alice "), search_term("bob_smith")];
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1 WHERE ");
        let mut predicates = builder.separated(" AND ");

        PgPublicOrganizationListReader::push_handle_contains(&mut predicates, &handle_contains);

        let sql = builder.sql();
        let sql_text = sql.as_str();
        assert!(sql_text.contains(" WHERE handle_search_text IS NOT NULL"));
        assert_eq!(sql_text.matches(" AND handle_search_text LIKE ").count(), 2);
        assert!(sql_text.contains("likequery($1)"));
        assert!(sql_text.contains("likequery($2)"));
    }

    #[test]
    fn empty_handle_contains_adds_no_predicate() {
        let handle_contains = Vec::new();
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1");

        if !handle_contains.is_empty() {
            builder.push(" WHERE ");
            let mut predicates = builder.separated(" AND ");
            PgPublicOrganizationListReader::push_handle_contains(&mut predicates, &handle_contains);
        }

        assert_eq!(builder.sql(), "SELECT 1");
    }
}
