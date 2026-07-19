use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationJoinRequestList, OrganizationJoinRequestListCriteria,
    OrganizationJoinRequestListCursor, OrganizationJoinRequestListItem,
    OrganizationJoinRequestListItemStatus, OrganizationJoinRequestListOrganization,
    OrganizationJoinRequestListReader, OrganizationJoinRequestListReaderError,
    OrganizationJoinRequestListSortKey,
};
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};

use super::pg_organization_join_request_list_item_row::PgOrganizationJoinRequestListItemRow;
use super::pg_organization_join_request_list_organization_row::PgOrganizationJoinRequestListOrganizationRow;

/// PostgreSQL-backed organization join request list reader.
pub struct PgOrganizationJoinRequestListReader;

impl PgOrganizationJoinRequestListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationJoinRequestListItemStatus) -> &'static str {
        match status {
            OrganizationJoinRequestListItemStatus::Pending => "pending",
            OrganizationJoinRequestListItemStatus::Approved => "approved",
            OrganizationJoinRequestListItemStatus::Rejected => "rejected",
            OrganizationJoinRequestListItemStatus::Canceled => "canceled",
        }
    }

    fn push_status_filter(
        builder: &mut QueryBuilder<Postgres>,
        statuses: &[OrganizationJoinRequestListItemStatus],
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

    async fn read_organization(
        uow: &mut PgUnitOfWork,
        organization_id: OrganizationId,
    ) -> Result<OrganizationJoinRequestListOrganization, OrganizationJoinRequestListReaderError>
    {
        let row = sqlx::query_as::<_, PgOrganizationJoinRequestListOrganizationRow>(
            r#"
            SELECT organization_id, handle, display_name, picture_type,
                   picture_object_name, picture_external_url, source_event_id,
                   updated_event_id
              FROM organization_join_request_list_organizations
             WHERE organization_id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationJoinRequestListReaderError::Persistence(Box::new(error)))?;

        OrganizationJoinRequestListOrganization::try_from(row)
            .map_err(|error| OrganizationJoinRequestListReaderError::Persistence(Box::new(error)))
    }
}

impl Default for PgOrganizationJoinRequestListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationJoinRequestListReader for PgOrganizationJoinRequestListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationJoinRequestListCriteria,
        cursor_options: Option<
            CursorOptions<OrganizationJoinRequestListSortKey, OrganizationJoinRequestListCursor>,
        >,
        limit: PageSize,
    ) -> Result<OrganizationJoinRequestList, OrganizationJoinRequestListReaderError> {
        let organization = Self::read_organization(uow, organization_id).await?;
        let query_limit = i64::from(limit.value()) + 1;
        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OrganizationJoinRequestListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);
        let page_cursor = cursor_options.and_then(|options| options.cursor);
        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id AS join_request_id,
                i.requester_user_id,
                i.status,
                i.created_at,
                i.source_event_id,
                i.updated_event_id,
                u.username AS requester_username,
                u.display_name AS requester_display_name,
                u.picture_type AS requester_picture_type,
                u.picture_object_name AS requester_picture_object_name,
                u.picture_external_url AS requester_picture_external_url,
                u.source_event_id AS requester_source_event_id,
                u.updated_event_id AS requester_updated_event_id
            FROM organization_join_request_list_items AS i
            INNER JOIN organization_join_request_list_users AS u
                    ON u.user_id = i.requester_user_id
            WHERE i.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value());
        if let Some(statuses) = criteria.statuses.as_deref() {
            Self::push_status_filter(&mut builder, statuses);
        }
        if let Some(cursor) = page_cursor {
            match (sort_key, sort_direction) {
                (OrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.created_at, i.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.join_request_id.value())
                        .push(")");
                }
                (OrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.created_at, i.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.join_request_id.value())
                        .push(")");
                }
                (OrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Asc) => {
                    builder
                        .push(" AND i.id > ")
                        .push_bind(cursor.join_request_id.value());
                }
                (OrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Desc) => {
                    builder
                        .push(" AND i.id < ")
                        .push_bind(cursor.join_request_id.value());
                }
            }
        }
        match (sort_key, sort_direction) {
            (OrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.created_at ASC, i.id ASC");
            }
            (OrganizationJoinRequestListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.created_at DESC, i.id DESC");
            }
            (OrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (OrganizationJoinRequestListSortKey::JoinRequestId, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }
        builder.push(" LIMIT ").push_bind(query_limit);
        let rows = builder
            .build_query_as::<PgOrganizationJoinRequestListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| {
                OrganizationJoinRequestListReaderError::Persistence(Box::new(error))
            })?;
        let page_limit = limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(OrganizationJoinRequestListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                OrganizationJoinRequestListReaderError::Persistence(Box::new(error))
            })?;
        let next_cursor = if has_next {
            items.last().map(|item| OrganizationJoinRequestListCursor {
                created_at: item.created_at,
                join_request_id: item.join_request_id,
            })
        } else {
            None
        };
        Ok(OrganizationJoinRequestList {
            organization,
            items,
            next_cursor,
        })
    }
}
