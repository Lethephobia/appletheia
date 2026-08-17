use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{OrganizationFragment, UserFragment};
use banking_iam_domain::{OrganizationId, UserId};

use super::PgFragmentLoaderError;
use super::organization_fragment::PgOrganizationFragmentRow;
use super::user_fragment::PgUserFragmentRow;

/// Materializes complete IAM fragment dependency graphs from normalized tables.
pub struct PgFragmentLoader;

impl PgFragmentLoader {
    pub async fn load_user(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<Option<UserFragment>, PgFragmentLoaderError> {
        let row = sqlx::query_as::<_, PgUserFragmentRow>(
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
                source_event_id,
                updated_event_id
            FROM user_fragments
            WHERE id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PgFragmentLoaderError::Persistence(Box::new(error)))?;

        row.map(UserFragment::try_from)
            .transpose()
            .map_err(|error| PgFragmentLoaderError::Persistence(Box::new(error)))
    }

    pub async fn load_required_user(
        uow: &mut PgUnitOfWork,
        user_id: UserId,
    ) -> Result<UserFragment, PgFragmentLoaderError> {
        Self::load_user(uow, user_id)
            .await?
            .ok_or(PgFragmentLoaderError::UserNotFound { user_id })
    }

    pub async fn load_organization(
        uow: &mut PgUnitOfWork,
        organization_id: OrganizationId,
    ) -> Result<Option<OrganizationFragment>, PgFragmentLoaderError> {
        let row = sqlx::query_as::<_, PgOrganizationFragmentRow>(
            r#"
            SELECT
                id,
                owner_user_id,
                owner_since,
                owner_source_event_id,
                owner_updated_event_id,
                handle,
                display_name,
                description,
                website_url,
                picture_type,
                picture_object_name,
                picture_external_url,
                created_at,
                source_event_id,
                updated_event_id
            FROM organization_fragments
            WHERE id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| PgFragmentLoaderError::Persistence(Box::new(error)))?;
        let Some(organization_row) = row else {
            return Ok(None);
        };
        let owner_user_id = UserId::try_from_uuid(organization_row.owner_user_id)
            .map_err(|error| PgFragmentLoaderError::Persistence(Box::new(error)))?;
        let owner = Self::load_required_user(uow, owner_user_id).await?;
        let organization = organization_row
            .try_into_fragment(owner)
            .map_err(|error| PgFragmentLoaderError::Persistence(Box::new(error)))?;

        Ok(Some(organization))
    }
}
