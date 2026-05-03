use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationProjectionStore, OrganizationProjectionStoreError, OrganizationProjectionUpsert,
};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationOwner, OrganizationPictureRef, OrganizationWebsiteUrl,
};
use sqlx::types::Json;

/// PostgreSQL-backed organization projection store.
pub struct PgOrganizationProjectionStore;

impl PgOrganizationProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: OrganizationOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            OrganizationOwner::User(user_id) => ("user", user_id.value()),
        }
    }
}

impl Default for PgOrganizationProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationProjectionStore for PgOrganizationProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(input.owner);

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
                created_at, updated_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                description = EXCLUDED.description,
                website_url = EXCLUDED.website_url,
                picture = EXCLUDED.picture,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(input.handle.value())
        .bind(input.display_name.value())
        .bind(
            input
                .description
                .as_ref()
                .map(OrganizationDescription::value),
        )
        .bind(
            input
                .website_url
                .as_ref()
                .map(|value| value.value().as_str()),
        )
        .bind(input.picture.as_ref().map(Json))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        owner: OrganizationOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            UPDATE organizations
               SET owner_type = $2,
                   owner_id = $3,
                   updated_at = $4,
                   updated_event_sequence = $5
             WHERE id = $1
               AND updated_event_sequence < $5
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET handle = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET display_name = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        description: Option<OrganizationDescription>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET description = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(OrganizationDescription::value))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_website_url(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        website_url: Option<OrganizationWebsiteUrl>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET website_url = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(website_url.as_ref().map(|value| value.value().as_str()))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organizations
               SET picture = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(picture.as_ref().map(Json))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM organizations
             WHERE id = $1
               AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
