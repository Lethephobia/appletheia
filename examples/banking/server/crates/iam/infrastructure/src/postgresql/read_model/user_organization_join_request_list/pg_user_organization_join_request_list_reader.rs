use appletheia::application::read_model::pagination::{CursorWindow, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserOrganizationJoinRequestList, UserOrganizationJoinRequestListCriteria,
    UserOrganizationJoinRequestListCursor, UserOrganizationJoinRequestListItem,
    UserOrganizationJoinRequestListReader, UserOrganizationJoinRequestListReaderError,
    UserOrganizationJoinRequestListSortKey, UserOrganizationJoinRequestListUser,
};
use banking_iam_domain::OrganizationJoinRequestStatus;
use banking_iam_domain::UserId;
use sqlx::{Postgres, QueryBuilder};

use super::pg_user_organization_join_request_list_item_row::PgUserOrganizationJoinRequestListItemRow;
use super::pg_user_organization_join_request_list_user_row::PgUserOrganizationJoinRequestListUserRow;

/// PostgreSQL-backed user organization join request list reader.
pub struct PgUserOrganizationJoinRequestListReader;

impl PgUserOrganizationJoinRequestListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationJoinRequestStatus) -> &'static str {
        match status {
            OrganizationJoinRequestStatus::Pending => "pending",
            OrganizationJoinRequestStatus::Approved => "approved",
            OrganizationJoinRequestStatus::Rejected => "rejected",
            OrganizationJoinRequestStatus::Canceled => "canceled",
        }
    }

    fn push_status_in(
        builder: &mut QueryBuilder<Postgres>,
        status_in: &[OrganizationJoinRequestStatus],
    ) {
        if status_in.is_empty() {
            builder.push(" AND FALSE");
            return;
        }
        builder.push(" AND i.status IN (");
        let mut separated = builder.separated(", ");
        for status in status_in {
            separated.push_bind_unseparated(Self::status_name(*status));
        }
        separated.push_unseparated(")");
    }

    async fn read_user(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<UserOrganizationJoinRequestListUser, UserOrganizationJoinRequestListReaderError>
    {
        let row = sqlx::query_as::<_, PgUserOrganizationJoinRequestListUserRow>(
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
        .map_err(|error| {
            UserOrganizationJoinRequestListReaderError::Persistence(Box::new(error))
        })?;

        UserOrganizationJoinRequestListUser::try_from(row).map_err(|error| {
            UserOrganizationJoinRequestListReaderError::Persistence(Box::new(error))
        })
    }
}

impl Default for PgUserOrganizationJoinRequestListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl UserOrganizationJoinRequestListReader for PgUserOrganizationJoinRequestListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        criteria: UserOrganizationJoinRequestListCriteria,
        sort: Sort<UserOrganizationJoinRequestListSortKey>,
        page: CursorWindow<UserOrganizationJoinRequestListCursor>,
    ) -> Result<UserOrganizationJoinRequestList, UserOrganizationJoinRequestListReaderError> {
        let user = Self::read_user(uow, user_id).await?;
        let query_limit = i64::from(page.limit().value()) + 1;
        let query_direction = page.query_direction(sort.direction);
        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id AS join_request_id,
                i.organization_id,
                i.status,
                i.created_at,
                i.source_event_id,
                i.updated_event_id,
                o.handle AS organization_handle,
                o.display_name AS organization_display_name,
                o.picture_type AS organization_picture_type,
                o.picture_object_name AS organization_picture_object_name,
                o.picture_external_url AS organization_picture_external_url,
                o.source_event_id AS organization_source_event_id,
                o.updated_event_id AS organization_updated_event_id
            FROM organization_join_request_fragments AS i
            INNER JOIN organization_fragments AS o
                    ON o.id = i.organization_id
            WHERE i.requester_user_id =
            "#,
        );
        builder.push_bind(user_id.value());
        if let Some(status_in) = criteria.status_in.as_deref() {
            Self::push_status_in(&mut builder, status_in);
        }
        if let Some(cursor) = page.boundary().copied() {
            match (sort.key, query_direction) {
                (UserOrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.created_at, i.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.join_request_id.value())
                        .push(")");
                }
                (UserOrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.created_at, i.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.join_request_id.value())
                        .push(")");
                }
                (UserOrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Asc) => {
                    builder
                        .push(" AND i.id > ")
                        .push_bind(cursor.join_request_id.value());
                }
                (UserOrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Desc) => {
                    builder
                        .push(" AND i.id < ")
                        .push_bind(cursor.join_request_id.value());
                }
            }
        }
        match (sort.key, query_direction) {
            (UserOrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.created_at ASC, i.id ASC");
            }
            (UserOrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.created_at DESC, i.id DESC");
            }
            (UserOrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (UserOrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }
        builder.push(" LIMIT ").push_bind(query_limit);
        let rows = builder
            .build_query_as::<PgUserOrganizationJoinRequestListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| {
                UserOrganizationJoinRequestListReaderError::Persistence(Box::new(error))
            })?;
        let page_limit = page.limit().value() as usize;
        let has_more = rows.len() > page_limit;
        let mut items = rows
            .into_iter()
            .take(page_limit)
            .map(UserOrganizationJoinRequestListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                UserOrganizationJoinRequestListReaderError::Persistence(Box::new(error))
            })?;
        if page.is_backward() {
            items.reverse();
        }
        let start_cursor = items
            .first()
            .map(|item| UserOrganizationJoinRequestListCursor {
                created_at: item.created_at,
                join_request_id: item.join_request_id,
            });
        let end_cursor = items
            .last()
            .map(|item| UserOrganizationJoinRequestListCursor {
                created_at: item.created_at,
                join_request_id: item.join_request_id,
            });
        let (has_previous, has_next) = if page.is_backward() {
            (has_more, !items.is_empty() && page.boundary().is_some())
        } else {
            (!items.is_empty() && page.boundary().is_some(), has_more)
        };
        Ok(UserOrganizationJoinRequestList {
            user,
            items,
            start_cursor,
            end_cursor,
            has_previous,
            has_next,
        })
    }
}
