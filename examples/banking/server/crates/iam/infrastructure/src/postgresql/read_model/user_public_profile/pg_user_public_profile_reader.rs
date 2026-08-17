use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserFragment, UserPublicProfile, UserPublicProfileReader, UserPublicProfileReaderError,
};
use banking_iam_domain::UserId;

use super::super::super::projection::PgUserFragmentRow;

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
        let row = sqlx::query_as::<_, PgUserFragmentRow>(
            r#"
            SELECT
                id AS user_id,
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
              FROM user_fragments u
             WHERE u.id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPublicProfileReaderError::Persistence(Box::new(e)))?;

        row.map(UserFragment::try_from)
            .transpose()
            .map(|fragment| fragment.map(UserPublicProfile::from))
            .map_err(|e| UserPublicProfileReaderError::Persistence(Box::new(e)))
    }
}
