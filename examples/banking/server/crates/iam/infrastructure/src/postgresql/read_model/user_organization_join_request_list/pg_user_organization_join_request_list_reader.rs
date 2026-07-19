use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserOrganizationJoinRequestList, UserOrganizationJoinRequestListCriteria,
    UserOrganizationJoinRequestListCursor, UserOrganizationJoinRequestListItem,
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListReader,
    UserOrganizationJoinRequestListReaderError, UserOrganizationJoinRequestListSortKey,
    UserOrganizationJoinRequestListUser,
};
use banking_iam_domain::UserId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};

use super::pg_user_organization_join_request_list_item_row::PgUserOrganizationJoinRequestListItemRow;
use super::pg_user_organization_join_request_list_user_row::PgUserOrganizationJoinRequestListUserRow;

/// PostgreSQL-backed user organization join request list reader.
pub struct PgUserOrganizationJoinRequestListReader;

impl PgUserOrganizationJoinRequestListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserOrganizationJoinRequestListItemStatus) -> &'static str {
        match status {
            UserOrganizationJoinRequestListItemStatus::Pending => "pending",
            UserOrganizationJoinRequestListItemStatus::Approved => "approved",
            UserOrganizationJoinRequestListItemStatus::Rejected => "rejected",
            UserOrganizationJoinRequestListItemStatus::Canceled => "canceled",
        }
    }

    fn push_status_filter(
        builder: &mut QueryBuilder<Postgres>,
        statuses: &[UserOrganizationJoinRequestListItemStatus],
    ) {
        if statuses.is_empty() {
            builder.push(" AND FALSE");
            return;
        }
        builder.push(" AND i.status IN (");
        let mut separated = builder.separated(", ");
        for status in statuses {
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
            SELECT user_id, username, display_name, picture_type, picture_object_name,
                   picture_external_url, source_event_id, updated_event_id
              FROM user_organization_join_request_list_users
             WHERE user_id = $1
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
        cursor_options: Option<
            CursorOptions<
                UserOrganizationJoinRequestListSortKey,
                UserOrganizationJoinRequestListCursor,
            >,
        >,
        page_size: PageSize,
    ) -> Result<UserOrganizationJoinRequestList, UserOrganizationJoinRequestListReaderError> {
        let user = Self::read_user(uow, user_id).await?;
        let query_limit = i64::from(page_size.value()) + 1;
        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(UserOrganizationJoinRequestListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);
        let page_cursor = cursor_options.and_then(|options| options.cursor);
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
            FROM user_organization_join_request_list_items AS i
            INNER JOIN user_organization_join_request_list_organizations AS o
                    ON o.organization_id = i.organization_id
            WHERE i.requester_user_id =
            "#,
        );
        builder.push_bind(user_id.value());
        if let Some(statuses) = criteria.statuses.as_deref() {
            Self::push_status_filter(&mut builder, statuses);
        }
        if let Some(cursor) = page_cursor {
            match (sort_key, sort_direction) {
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
        match (sort_key, sort_direction) {
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
        let output_limit = page_size.value() as usize;
        let has_next = rows.len() > output_limit;
        let items = rows
            .into_iter()
            .take(output_limit)
            .map(UserOrganizationJoinRequestListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                UserOrganizationJoinRequestListReaderError::Persistence(Box::new(error))
            })?;
        let next_cursor = if has_next {
            items
                .last()
                .map(|item| UserOrganizationJoinRequestListCursor {
                    created_at: item.created_at,
                    join_request_id: item.join_request_id,
                })
        } else {
            None
        };
        Ok(UserOrganizationJoinRequestList {
            user,
            items,
            next_cursor,
        })
    }
}
