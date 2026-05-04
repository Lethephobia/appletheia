use appletheia::application::event::EventSequence;
use appletheia::domain::{AggregateId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_application::{
    CurrencyListItemStatus, CurrencyListItemWriter, CurrencyListItemWriterError,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencySymbol,
};

/// PostgreSQL-backed currency list item writer.
pub struct PgCurrencyListItemWriter;

impl PgCurrencyListItemWriter {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: CurrencyOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            CurrencyOwner::User(user_id) => ("user", user_id.value()),
            CurrencyOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: CurrencyListItemStatus) -> &'static str {
        match status {
            CurrencyListItemStatus::Active => "active",
            CurrencyListItemStatus::Inactive => "inactive",
        }
    }
}

impl Default for PgCurrencyListItemWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyListItemWriter for PgCurrencyListItemWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        supply: CurrencyAmount,
        status: CurrencyListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            INSERT INTO currency_list_items (
                id, owner_type, owner_id, symbol, name, decimals, supply, status, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                symbol = EXCLUDED.symbol,
                name = EXCLUDED.name,
                decimals = EXCLUDED.decimals,
                supply = EXCLUDED.supply,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE currency_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(symbol.value())
        .bind(name.value())
        .bind(i16::from(decimals.value()))
        .bind(supply.value().to_string())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_currency_owner(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        owner: CurrencyOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET owner_type = $2, owner_id = $3, updated_at = $4,
                   updated_event_sequence = $5
             WHERE id = $1 AND updated_event_sequence < $5
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET symbol = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(symbol.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET name = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn increase_currency_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET supply = supply + $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn decrease_currency_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET supply = supply - $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        status: CurrencyListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_items
               SET status = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
        _occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM currency_list_items
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO currency_list_item_owner_users (
                id, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE currency_list_item_owner_users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_users
               SET username = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_users
               SET display_name = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_users
               SET picture = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(picture.map(sqlx::types::Json))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        _occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM currency_list_items
             WHERE owner_type = 'user'
               AND owner_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM currency_list_item_owner_users
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        display_name: OrganizationDisplayName,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO currency_list_item_owner_organizations (
                id, handle, display_name, picture, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture = EXCLUDED.picture,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE currency_list_item_owner_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(display_name.value())
        .bind(picture.map(sqlx::types::Json))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_organizations
               SET handle = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_organizations
               SET display_name = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE currency_list_item_owner_organizations
               SET picture = $2, updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(picture.map(sqlx::types::Json))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
        _occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM currency_list_items
             WHERE owner_type = 'organization'
               AND owner_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM currency_list_item_owner_organizations
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
