use appletheia::application::read_model::pagination::{CursorPage, Sort, SortDirection};
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    InternalOrganizationSummaryPart, OrganizationInvitationList,
    OrganizationInvitationListCriteria, OrganizationInvitationListCursor,
    OrganizationInvitationListItemPart, OrganizationInvitationListReader,
    OrganizationInvitationListReaderError, OrganizationInvitationListSortKey,
};
use banking_iam_domain::OrganizationId;
use banking_iam_domain::OrganizationInvitationStatus;
use sqlx::{Postgres, QueryBuilder};

use super::pg_organization_invitation_list_item_row::PgOrganizationInvitationListItemRow;
use super::pg_organization_invitation_list_organization_row::PgOrganizationInvitationListOrganizationRow;

/// PostgreSQL-backed organization invitation list reader.
pub struct PgOrganizationInvitationListReader;

impl PgOrganizationInvitationListReader {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationInvitationStatus) -> &'static str {
        match status {
            OrganizationInvitationStatus::Pending => "pending",
            OrganizationInvitationStatus::Accepted => "accepted",
            OrganizationInvitationStatus::Declined => "declined",
            OrganizationInvitationStatus::Canceled => "canceled",
            OrganizationInvitationStatus::Rejected => "rejected",
        }
    }

    fn push_status_in(
        builder: &mut QueryBuilder<Postgres>,
        status_in: &[OrganizationInvitationStatus],
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
    ) -> Result<InternalOrganizationSummaryPart, OrganizationInvitationListReaderError> {
        let row = sqlx::query_as::<_, PgOrganizationInvitationListOrganizationRow>(
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
        .map_err(|error| OrganizationInvitationListReaderError::Persistence(Box::new(error)))?;

        InternalOrganizationSummaryPart::try_from(row)
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
        sort: Sort<OrganizationInvitationListSortKey>,
        page: CursorPage<OrganizationInvitationListCursor>,
    ) -> Result<OrganizationInvitationList, OrganizationInvitationListReaderError> {
        let organization = Self::read_organization(uow, organization_id).await?;
        let query_limit = i64::from(page.limit.value()) + 1;

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
            FROM organization_invitation_fragments AS i
            INNER JOIN user_fragments AS u
                    ON u.id = i.invitee_user_id
            WHERE i.organization_id =
            "#,
        );
        builder.push_bind(organization_id.value());

        if let Some(status_in) = criteria.status_in.as_deref() {
            Self::push_status_in(&mut builder, status_in);
        }

        if let Some(cursor) = page.after {
            match (sort.key, sort.direction) {
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

        match (sort.key, sort.direction) {
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
        let page_limit = page.limit.value() as usize;
        let has_next = rows.len() > page_limit;
        let items = rows
            .into_iter()
            .take(page_limit)
            .map(OrganizationInvitationListItemPart::try_from)
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
