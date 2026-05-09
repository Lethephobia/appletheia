use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{UserPrivateInfo, UserPrivateInfoReader, UserPrivateInfoReaderError};
use banking_iam_domain::UserId;

use super::pg_user_private_info_identity_row::PgUserPrivateInfoIdentityRow;
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
                created_at
              FROM user_private_infos
             WHERE id = $1
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
            SELECT provider, subject, email
              FROM user_private_info_identities
             WHERE user_id = $1
             ORDER BY provider ASC, subject ASC
            "#,
        )
        .bind(user_id.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        let user_private_info = user_row
            .into_user_private_info(identity_rows)
            .map_err(|e| UserPrivateInfoReaderError::Persistence(Box::new(e)))?;

        Ok(Some(user_private_info))
    }
}
