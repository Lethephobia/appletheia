use appletheia::application::read_model::pagination::{CursorPage, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    InternalOrganizationSummaryPart, OrganizationJoinRequestList,
    OrganizationJoinRequestListCriteria, OrganizationJoinRequestListCursor,
    OrganizationJoinRequestListItemPart, OrganizationJoinRequestListReader,
    OrganizationJoinRequestListReaderError, OrganizationJoinRequestListSortKey,
};
use banking_iam_domain::OrganizationId;
use banking_iam_domain::OrganizationJoinRequestStatus;
use sqlx::{Postgres, QueryBuilder};

use super::pg_organization_join_request_list_item_row::PgOrganizationJoinRequestListItemRow;
use super::pg_organization_join_request_list_organization_row::PgOrganizationJoinRequestListOrganizationRow;

/// PostgreSQL-backed organization join request list reader.
pub struct PgOrganizationJoinRequestListReader;

impl PgOrganizationJoinRequestListReader {
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

    async fn read_organization(
        uow: &mut PgUnitOfWork,
        organization_id: OrganizationId,
    ) -> Result<InternalOrganizationSummaryPart, OrganizationJoinRequestListReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationJoinRequestListOrganizationRow>(
            r#"
            SELECT id AS organization_id, handle, display_name, picture_type,
                   picture_object_name, picture_external_url, source_event_id,
                   updated_event_id
              FROM organization_fragments
             WHERE id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationJoinRequestListReaderError::Persistence(Box::new(error)))?;

        InternalOrganizationSummaryPart::try_from(row)
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
        sort: Sort<OrganizationJoinRequestListSortKey>,
        page: CursorPage<OrganizationJoinRequestListCursor>,
    ) -> Result<OrganizationJoinRequestList, OrganizationJoinRequestListReaderError> {
        let organization = Self::read_organization(uow, organization_id).await?;
        let query_limit = i64::from(page.limit.value()) + 1;
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
            FROM organization_join_request_fragments AS i
            INNER JOIN user_fragments AS u
                    ON u.id = i.requester_user_id
            WHERE i.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value());
        if let Some(status_in) = criteria.status_in.as_deref() {
            Self::push_status_in(&mut builder, status_in);
        }
        if let Some(cursor) = page.after {
            match (sort.key, sort.direction) {
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
        match (sort.key, sort.direction) {
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
        let page_limit = page.limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(OrganizationJoinRequestListItemPart::try_from)
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
