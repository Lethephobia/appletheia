use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationView, OrganizationViewStore, OrganizationViewStoreError,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationOwner, OrganizationPictureRef, OrganizationWebsiteUrl,
};
use sqlx::types::Json;

/// PostgreSQL-backed organization view store.
pub struct PgOrganizationViewStore;

impl PgOrganizationViewStore {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: OrganizationOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            OrganizationOwner::User(user_id) => ("user", user_id.value()),
        }
    }
}

impl Default for PgOrganizationViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationViewStore for PgOrganizationViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        view: OrganizationView,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            INSERT INTO organizations (
                id,
                owner_type,
                owner_id,
                handle,
                display_name,
                description,
                website_url,
                picture,
                updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                description = EXCLUDED.description,
                website_url = EXCLUDED.website_url,
                picture = EXCLUDED.picture,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(view.id.value())
        .bind(Self::owner_parts(view.owner).0)
        .bind(Self::owner_parts(view.owner).1)
        .bind(view.handle.value())
        .bind(view.display_name.value())
        .bind(
            view.description
                .as_ref()
                .map(OrganizationDescription::value),
        )
        .bind(
            view.website_url
                .as_ref()
                .map(|value| value.value().as_str()),
        )
        .bind(view.picture.as_ref().map(Json))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        owner: OrganizationOwner,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            UPDATE organizations
               SET owner_type = $2,
                   owner_id = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET handle = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET display_name = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET description = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(OrganizationDescription::value))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET website_url = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(website_url.as_ref().map(|value| value.value().as_str()))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET picture = $2,
                   updated_event_sequence = $3
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(picture.as_ref().map(Json))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
    ) -> Result<(), OrganizationViewStoreError> {
        sqlx::query(
            r#"
            DELETE FROM organizations
             WHERE id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
