use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{UserPrivateInfo, UserPrivateInfoReader, UserPrivateInfoReaderError};
use banking_iam_domain::UserId;

use super::pg_user_private_info_identity_row::PgUserPrivateInfoIdentityRow;
use super::pg_user_private_info_organization_membership_row::PgUserPrivateInfoOrganizationMembershipRow;
use super::pg_user_private_info_row::PgUserPrivateInfoRow;

/// PostgreSQL-backed user-private information reader.
pub struct PgUserPrivateInfoReader;

impl PgUserPrivateInfoReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUserPrivateInfoReader {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPrivateInfoReader for PgUserPrivateInfoReader {
    type Uow = PgUnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<Option<UserPrivateInfo>, UserPrivateInfoReaderError> {
        let user_row = sqlx::query_as::<_, PgUserPrivateInfoRow>(
            r#"
            SELECT
                id,
                username,
                display_name,
                bio,
                picture_type,
                picture_object_name,
                picture_external_url,
                status,
                created_at,
                u.source_event_id,
                u.updated_event_id
              FROM user_private_infos u
             WHERE u.id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        let Some(user_row) = user_row else {
            return Ok(None);
        };

        let identity_rows = sqlx::query_as::<_, PgUserPrivateInfoIdentityRow>(
            r#"
            SELECT
                i.provider,
                i.subject,
                i.email,
                i.source_event_id,
                i.updated_event_id
              FROM user_private_info_identities i
             WHERE i.user_id = $1
             ORDER BY i.provider ASC, i.subject ASC
            "#,
        )
        .bind(user_id.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        let organization_membership_rows =
            sqlx::query_as::<_, PgUserPrivateInfoOrganizationMembershipRow>(
                r#"
            SELECT
                m.organization_id,
                m.roles::text AS roles,
                m.source_event_id,
                m.updated_event_id,
                o.handle AS organization_handle,
                o.display_name AS organization_display_name,
                o.picture_type AS organization_picture_type,
                o.picture_object_name AS organization_picture_object_name,
                o.picture_external_url AS organization_picture_external_url,
                o.source_event_id AS organization_source_event_id,
                o.updated_event_id AS organization_updated_event_id
              FROM user_private_info_organization_memberships m
              LEFT JOIN user_private_info_organizations o ON o.id = m.organization_id
             WHERE m.user_id = $1
             ORDER BY o.handle ASC NULLS LAST, m.organization_id ASC
            "#,
            )
            .bind(user_id.value())
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        let user_private_info = user_row
            .into_user_private_info(identity_rows, organization_membership_rows)
            .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        Ok(Some(user_private_info))
    }
}
