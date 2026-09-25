use appletheia::application::read_model::pagination::{CursorWindow, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserOrganizationMembershipList, UserOrganizationMembershipListCursor,
    UserOrganizationMembershipListItem, UserOrganizationMembershipListReader,
    UserOrganizationMembershipListReaderError, UserOrganizationMembershipListSortKey,
    UserOrganizationMembershipListUser,
};
use banking_iam_domain::UserId;
use sqlx::{Postgres, QueryBuilder};

use super::pg_user_organization_membership_list_item_row::PgUserOrganizationMembershipListItemRow;
use super::pg_user_organization_membership_list_user_row::PgUserOrganizationMembershipListUserRow;

/// PostgreSQL-backed user organization membership list reader.
pub struct PgUserOrganizationMembershipListReader;

impl PgUserOrganizationMembershipListReader {
    pub fn new() -> Self {
        Self
    }

    async fn read_user(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<UserOrganizationMembershipListUser, UserOrganizationMembershipListReaderError> {
        let row = sqlx::query_as::<_, PgUserOrganizationMembershipListUserRow>(
            r#"
            SELECT id AS user_id, username, display_name, picture_type, picture_object_name,
                   picture_external_url, source_event_id, updated_event_id
              FROM user_fragments
             WHERE id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationMembershipListReaderError::Persistence(Box::new(error)))?;

        UserOrganizationMembershipListUser::try_from(row).map_err(|error| {
            UserOrganizationMembershipListReaderError::Persistence(Box::new(error))
        })
    }
}

impl Default for PgUserOrganizationMembershipListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl UserOrganizationMembershipListReader for PgUserOrganizationMembershipListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        sort: Sort<UserOrganizationMembershipListSortKey>,
        page: CursorWindow<UserOrganizationMembershipListCursor>,
    ) -> Result<UserOrganizationMembershipList, UserOrganizationMembershipListReaderError> {
        let user = Self::read_user(uow, user_id).await?;
        let query_limit = i64::from(page.limit().value()) + 1;
        let query_direction = page.query_direction(sort.direction);
        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                m.organization_membership_id,
                m.organization_id,
                m.roles::text AS roles,
                m.created_at,
                m.source_event_id,
                m.updated_event_id,
                o.handle AS organization_handle,
                o.display_name AS organization_display_name,
                o.picture_type AS organization_picture_type,
                o.picture_object_name AS organization_picture_object_name,
                o.picture_external_url AS organization_picture_external_url,
                o.source_event_id AS organization_source_event_id,
                o.updated_event_id AS organization_updated_event_id
            FROM organization_membership_fragments AS m
            INNER JOIN organization_fragments AS o
                    ON o.id = m.organization_id
            WHERE m.user_id =
            "#,
        );
        builder.push_bind(user_id.value());
        if let Some(cursor) = page.boundary().copied() {
            match (sort.key, query_direction) {
                (UserOrganizationMembershipListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (m.created_at, m.organization_membership_id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.organization_membership_id.value())
                        .push(")");
                }
                (UserOrganizationMembershipListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (m.created_at, m.organization_membership_id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.organization_membership_id.value())
                        .push(")");
                }
                (
                    UserOrganizationMembershipListSortKey::OrganizationMembershipId,
                    SortDirection::Asc,
                ) => {
                    builder
                        .push(" AND m.organization_membership_id > ")
                        .push_bind(cursor.organization_membership_id.value());
                }
                (
                    UserOrganizationMembershipListSortKey::OrganizationMembershipId,
                    SortDirection::Desc,
                ) => {
                    builder
                        .push(" AND m.organization_membership_id < ")
                        .push_bind(cursor.organization_membership_id.value());
                }
            }
        }
        match (sort.key, query_direction) {
            (UserOrganizationMembershipListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY m.created_at ASC, m.organization_membership_id ASC");
            }
            (UserOrganizationMembershipListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY m.created_at DESC, m.organization_membership_id DESC");
            }
            (
                UserOrganizationMembershipListSortKey::OrganizationMembershipId,
                SortDirection::Asc,
            ) => {
                builder.push(" ORDER BY m.organization_membership_id ASC");
            }
            (
                UserOrganizationMembershipListSortKey::OrganizationMembershipId,
                SortDirection::Desc,
            ) => {
                builder.push(" ORDER BY m.organization_membership_id DESC");
            }
        }
        builder.push(" LIMIT ").push_bind(query_limit);
        let rows = builder
            .build_query_as::<PgUserOrganizationMembershipListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| {
                UserOrganizationMembershipListReaderError::Persistence(Box::new(error))
            })?;
        let page_limit = page.limit().value() as usize;
        let has_more = rows.len() > page_limit;
        let mut items = rows
            .into_iter()
            .take(page_limit)
            .map(UserOrganizationMembershipListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                UserOrganizationMembershipListReaderError::Persistence(Box::new(error))
            })?;
        if page.is_backward() {
            items.reverse();
        }
        let start_cursor = items
            .first()
            .map(|item| UserOrganizationMembershipListCursor {
                created_at: item.created_at,
                organization_membership_id: item.organization_membership_id,
            });
        let end_cursor = items
            .last()
            .map(|item| UserOrganizationMembershipListCursor {
                created_at: item.created_at,
                organization_membership_id: item.organization_membership_id,
            });
        let (has_previous, has_next) = if page.is_backward() {
            (has_more, !items.is_empty() && page.boundary().is_some())
        } else {
            (!items.is_empty() && page.boundary().is_some(), has_more)
        };
        Ok(UserOrganizationMembershipList {
            user,
            items,
            start_cursor,
            end_cursor,
            has_previous,
            has_next,
        })
    }
}
