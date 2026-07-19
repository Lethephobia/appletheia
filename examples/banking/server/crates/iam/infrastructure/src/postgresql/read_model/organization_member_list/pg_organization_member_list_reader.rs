use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationMemberList, OrganizationMemberListCriteria, OrganizationMemberListCursor,
    OrganizationMemberListItem, OrganizationMemberListOrganization, OrganizationMemberListReader,
    OrganizationMemberListReaderError, OrganizationMemberListSortKey,
};
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};

use super::pg_organization_member_list_item_row::PgOrganizationMemberListItemRow;
use super::pg_organization_member_list_organization_row::PgOrganizationMemberListOrganizationRow;

/// PostgreSQL-backed organization member list reader.
pub struct PgOrganizationMemberListReader;

impl PgOrganizationMemberListReader {
    pub fn new() -> Self {
        Self
    }

    fn push_predicate_prefix(builder: &mut QueryBuilder<Postgres>, has_predicate: &mut bool) {
        if *has_predicate {
            builder.push(" AND ");
        } else {
            builder.push(" WHERE ");
            *has_predicate = true;
        }
    }

    fn push_username_contains(
        builder: &mut QueryBuilder<Postgres>,
        has_predicate: &mut bool,
        username_contains: &[String],
    ) {
        let mut pushed_search_presence = false;

        for contains in username_contains
            .iter()
            .filter(|contains| contains.chars().any(|character| !character.is_whitespace()))
        {
            if !pushed_search_presence {
                Self::push_predicate_prefix(builder, has_predicate);
                builder.push("u.username_search_text IS NOT NULL");
                pushed_search_presence = true;
            }

            Self::push_predicate_prefix(builder, has_predicate);
            builder
                .push("u.username_search_text LIKE likequery(regexp_replace(lower(")
                .push_bind(contains.as_str())
                .push("), '[[:space:]]+', '', 'g'))");
        }
    }

    async fn read_organization(
        uow: &mut PgUnitOfWork,
        organization_id: OrganizationId,
    ) -> Result<OrganizationMemberListOrganization, OrganizationMemberListReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationMemberListOrganizationRow>(
            r#"
            SELECT organization_id, handle, display_name, picture_type,
                   picture_object_name, picture_external_url, source_event_id,
                   updated_event_id
              FROM organization_member_list_organizations
             WHERE organization_id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListReaderError::Persistence(Box::new(error)))?;

        OrganizationMemberListOrganization::try_from(row)
            .map_err(|error| OrganizationMemberListReaderError::Persistence(Box::new(error)))
    }
}

impl Default for PgOrganizationMemberListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationMemberListReader for PgOrganizationMemberListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationMemberListCriteria,
        cursor_options: Option<
            CursorOptions<OrganizationMemberListSortKey, OrganizationMemberListCursor>,
        >,
        page_size: PageSize,
    ) -> Result<OrganizationMemberList, OrganizationMemberListReaderError> {
        let organization = Self::read_organization(uow, organization_id).await?;
        let query_limit = i64::from(page_size.value()) + 1;
        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OrganizationMemberListSortKey::JoinedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);
        let page_cursor = cursor_options.and_then(|options| options.cursor);

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            WITH member_rows AS (
                SELECT
                    m.user_id,
                    m.roles::text AS roles,
                    m.user_id = o.owner_user_id AS is_owner,
                    m.joined_at,
                    m.source_event_id,
                    m.updated_event_id
                FROM organization_member_list_memberships AS m
                INNER JOIN organization_member_list_organizations AS o
                        ON o.organization_id = m.organization_id
                WHERE m.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value()).push(
            r#"
                UNION ALL
                SELECT
                    o.owner_user_id AS user_id,
                    '[]'::text AS roles,
                    TRUE AS is_owner,
                    o.owner_since AS joined_at,
                    o.owner_source_event_id AS source_event_id,
                    o.owner_updated_event_id AS updated_event_id
                FROM organization_member_list_organizations AS o
                WHERE o.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value()).push(
            r#"
                  AND NOT EXISTS (
                      SELECT 1
                      FROM organization_member_list_memberships AS m
                      WHERE m.organization_id = o.organization_id
                        AND m.user_id = o.owner_user_id
                  )
            )
            SELECT
                r.user_id,
                u.username,
                u.display_name,
                u.picture_type,
                u.picture_object_name,
                u.picture_external_url,
                r.roles,
                r.is_owner,
                r.joined_at,
                r.source_event_id,
                r.updated_event_id,
                u.source_event_id AS member_source_event_id,
                u.updated_event_id AS member_updated_event_id
            FROM member_rows AS r
            INNER JOIN organization_member_list_users AS u ON u.user_id = r.user_id
            "#,
        );

        let mut has_predicate = false;
        Self::push_username_contains(
            &mut builder,
            &mut has_predicate,
            &criteria.username_contains,
        );

        if let Some(cursor) = page_cursor {
            Self::push_predicate_prefix(&mut builder, &mut has_predicate);
            match (sort_key, sort_direction) {
                (OrganizationMemberListSortKey::JoinedAt, SortDirection::Asc) => {
                    builder
                        .push("(r.joined_at, r.user_id) > (")
                        .push_bind(cursor.joined_at.value())
                        .push(", ")
                        .push_bind(cursor.user_id.value())
                        .push(")");
                }
                (OrganizationMemberListSortKey::JoinedAt, SortDirection::Desc) => {
                    builder
                        .push("(r.joined_at, r.user_id) < (")
                        .push_bind(cursor.joined_at.value())
                        .push(", ")
                        .push_bind(cursor.user_id.value())
                        .push(")");
                }
                (OrganizationMemberListSortKey::UserId, SortDirection::Asc) => {
                    builder
                        .push("r.user_id > ")
                        .push_bind(cursor.user_id.value());
                }
                (OrganizationMemberListSortKey::UserId, SortDirection::Desc) => {
                    builder
                        .push("r.user_id < ")
                        .push_bind(cursor.user_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (OrganizationMemberListSortKey::JoinedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY r.joined_at ASC, r.user_id ASC");
            }
            (OrganizationMemberListSortKey::JoinedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY r.joined_at DESC, r.user_id DESC");
            }
            (OrganizationMemberListSortKey::UserId, SortDirection::Asc) => {
                builder.push(" ORDER BY r.user_id ASC");
            }
            (OrganizationMemberListSortKey::UserId, SortDirection::Desc) => {
                builder.push(" ORDER BY r.user_id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgOrganizationMemberListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| OrganizationMemberListReaderError::Persistence(Box::new(error)))?;
        let output_limit = page_size.value() as usize;
        let has_next = rows.len() > output_limit;
        let items = rows
            .into_iter()
            .take(output_limit)
            .map(OrganizationMemberListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| OrganizationMemberListReaderError::Persistence(Box::new(error)))?;
        let next_cursor = if has_next {
            items.last().map(|item| OrganizationMemberListCursor {
                joined_at: item.joined_at,
                user_id: item.member.user_id,
            })
        } else {
            None
        };

        Ok(OrganizationMemberList {
            organization,
            items,
            next_cursor,
        })
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Postgres, QueryBuilder};

    use super::PgOrganizationMemberListReader;

    #[test]
    fn username_contains_adds_one_and_predicate_per_non_blank_value() {
        let username_contains = vec![
            " Alice ".to_owned(),
            "bob_smith".to_owned(),
            "   ".to_owned(),
        ];
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1");
        let mut has_predicate = false;

        PgOrganizationMemberListReader::push_username_contains(
            &mut builder,
            &mut has_predicate,
            &username_contains,
        );

        let sql = builder.sql();
        let sql_text = sql.as_str();
        assert!(sql_text.contains("WHERE u.username_search_text IS NOT NULL"));
        assert_eq!(
            sql_text
                .matches(" AND u.username_search_text LIKE ")
                .count(),
            2
        );
        assert!(sql_text.contains("lower($1)"));
        assert!(sql_text.contains("lower($2)"));
    }

    #[test]
    fn empty_username_contains_adds_no_predicate() {
        let username_contains = vec!["   ".to_owned()];
        let mut builder = QueryBuilder::<Postgres>::new("SELECT 1");
        let mut has_predicate = false;

        PgOrganizationMemberListReader::push_username_contains(
            &mut builder,
            &mut has_predicate,
            &username_contains,
        );

        assert_eq!(builder.sql(), "SELECT 1");
    }
}
