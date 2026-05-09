use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserPublicProfile, UserPublicProfileReader, UserPublicProfileReaderError,
};
use banking_iam_domain::UserId;

use super::pg_user_public_profile_row::PgUserPublicProfileRow;

/// PostgreSQL-backed public user profile reader.
pub struct PgUserPublicProfileReader;

impl PgUserPublicProfileReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUserPublicProfileReader {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPublicProfileReader for PgUserPublicProfileReader {
    type Uow = PgUnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<Option<UserPublicProfile>, UserPublicProfileReaderError> {
        let row = sqlx::query_as::<_, PgUserPublicProfileRow>(
            r#"
            SELECT
                id,
                username,
                display_name,
                bio,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at
              FROM user_public_profiles
             WHERE id = $1 AND status = 'active'
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileReaderError::Persistence(Box::new(e)))?;

        row.map(UserPublicProfile::try_from)
            .transpose()
            .map_err(|e| UserPublicProfileReaderError::Persistence(Box::new(e)))
    }
}
