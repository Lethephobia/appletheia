use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationInvitationList, OrganizationInvitationListCriteria,
    OrganizationInvitationListCursor, OrganizationInvitationListItem,
    OrganizationInvitationListItemStatus, OrganizationInvitationListOrganization,
    OrganizationInvitationListReader, OrganizationInvitationListReaderError,
    OrganizationInvitationListSortKey,
};
use banking_iam_domain::OrganizationId;
use banking_shared_kernel_application::read_model::{CursorOptions, PageSize, SortDirection};
use sqlx::{Postgres, QueryBuilder};

use super::pg_organization_invitation_list_item_row::PgOrganizationInvitationListItemRow;
use super::pg_organization_invitation_list_organization_row::PgOrganizationInvitationListOrganizationRow;

/// PostgreSQL-backed organization invitation list reader.
pub struct PgOrganizationInvitationListReader;

impl PgOrganizationInvitationListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationInvitationListItemStatus) -> &'static str {
        match status {
            OrganizationInvitationListItemStatus::Pending => "pending",
            OrganizationInvitationListItemStatus::Accepted => "accepted",
            OrganizationInvitationListItemStatus::Declined => "declined",
            OrganizationInvitationListItemStatus::Canceled => "canceled",
            OrganizationInvitationListItemStatus::Rejected => "rejected",
        }
    }

    fn push_status_filter(
        builder: &mut QueryBuilder<Postgres>,
        statuses: &[OrganizationInvitationListItemStatus],
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
    ) -> Result<OrganizationInvitationListOrganization, OrganizationInvitationListReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationInvitationListOrganizationRow>(
            r#"
            SELECT organization_id, handle, display_name, picture_type,
                   picture_object_name, picture_external_url, source_event_id,
                   updated_event_id
              FROM organization_invitation_list_organizations
             WHERE organization_id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationInvitationListReaderError::Persistence(Box::new(error)))?;

        OrganizationInvitationListOrganization::try_from(row)
            .map_err(|error| OrganizationInvitationListReaderError::Persistence(Box::new(error)))
    }
}

impl Default for PgOrganizationInvitationListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationInvitationListReader for PgOrganizationInvitationListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        criteria: OrganizationInvitationListCriteria,
        cursor_options: Option<
            CursorOptions<OrganizationInvitationListSortKey, OrganizationInvitationListCursor>,
        >,
        page_size: PageSize,
    ) -> Result<OrganizationInvitationList, OrganizationInvitationListReaderError> {
        let organization = Self::read_organization(uow, organization_id).await?;
        let query_limit = i64::from(page_size.value()) + 1;
        let sort_key = cursor_options
            .map(|options| options.sort_key)
            .unwrap_or(OrganizationInvitationListSortKey::CreatedAt);
        let sort_direction = cursor_options
            .map(|options| options.sort_direction)
            .unwrap_or(SortDirection::Desc);
        let page_cursor = cursor_options.and_then(|options| options.cursor);

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                i.id AS invitation_id,
                i.invitee_user_id,
                i.roles::text AS roles,
                i.issuer_type,
                i.issuer_user_id,
                i.expires_at,
                i.status,
                i.created_at,
                i.source_event_id,
                i.updated_event_id,
                u.username AS invitee_username,
                u.display_name AS invitee_display_name,
                u.picture_type AS invitee_picture_type,
                u.picture_object_name AS invitee_picture_object_name,
                u.picture_external_url AS invitee_picture_external_url,
                u.source_event_id AS invitee_source_event_id,
                u.updated_event_id AS invitee_updated_event_id
            FROM organization_invitation_list_items AS i
            INNER JOIN organization_invitation_list_users AS u
                    ON u.user_id = i.invitee_user_id
            WHERE i.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value());

        if let Some(statuses) = criteria.statuses.as_deref() {
            Self::push_status_filter(&mut builder, statuses);
        }

        if let Some(cursor) = page_cursor {
            match (sort_key, sort_direction) {
                (OrganizationInvitationListSortKey::CreatedAt, SortDirection::Asc) => {
                    builder
                        .push(" AND (i.created_at, i.id) > (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.invitation_id.value())
                        .push(")");
                }
                (OrganizationInvitationListSortKey::CreatedAt, SortDirection::Desc) => {
                    builder
                        .push(" AND (i.created_at, i.id) < (")
                        .push_bind(cursor.created_at.value())
                        .push(", ")
                        .push_bind(cursor.invitation_id.value())
                        .push(")");
                }
                (OrganizationInvitationListSortKey::InvitationId, SortDirection::Asc) => {
                    builder
                        .push(" AND i.id > ")
                        .push_bind(cursor.invitation_id.value());
                }
                (OrganizationInvitationListSortKey::InvitationId, SortDirection::Desc) => {
                    builder
                        .push(" AND i.id < ")
                        .push_bind(cursor.invitation_id.value());
                }
            }
        }

        match (sort_key, sort_direction) {
            (OrganizationInvitationListSortKey::CreatedAt, SortDirection::Asc) => {
                builder.push(" ORDER BY i.created_at ASC, i.id ASC");
            }
            (OrganizationInvitationListSortKey::CreatedAt, SortDirection::Desc) => {
                builder.push(" ORDER BY i.created_at DESC, i.id DESC");
            }
            (OrganizationInvitationListSortKey::InvitationId, SortDirection::Asc) => {
                builder.push(" ORDER BY i.id ASC");
            }
            (OrganizationInvitationListSortKey::InvitationId, SortDirection::Desc) => {
                builder.push(" ORDER BY i.id DESC");
            }
        }

        builder.push(" LIMIT ").push_bind(query_limit);

        let rows = builder
            .build_query_as::<PgOrganizationInvitationListItemRow>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| OrganizationInvitationListReaderError::Persistence(Box::new(error)))?;
        let output_limit = page_size.value() as usize;
        let has_next = rows.len() > output_limit;
        let items = rows
            .into_iter()
            .take(output_limit)
            .map(OrganizationInvitationListItem::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| OrganizationInvitationListReaderError::Persistence(Box::new(error)))?;
        let next_cursor = if has_next {
            items.last().map(|item| OrganizationInvitationListCursor {
                created_at: item.created_at,
                invitation_id: item.invitation_id,
            })
        } else {
            None
        };

        Ok(OrganizationInvitationList {
            organization,
            items,
            next_cursor,
        })
    }
}
