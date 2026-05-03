use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyProjectionStore, CurrencyProjectionStoreError, CurrencyProjectionUpsert,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyId, CurrencyName, CurrencyOwner, CurrencyStatus, CurrencySymbol,
};

/// PostgreSQL-backed currency projection store.
pub struct PgCurrencyProjectionStore;

impl PgCurrencyProjectionStore {
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

    fn status_name(status: CurrencyStatus) -> &'static str {
        match status {
            CurrencyStatus::Active => "active",
            CurrencyStatus::Inactive => "inactive",
            CurrencyStatus::Removed => "removed",
        }
    }
}

impl Default for PgCurrencyProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyProjectionStore for PgCurrencyProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: CurrencyProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(input.owner);

        sqlx::query(
            r#"
            INSERT INTO currencies (
                id, owner_type, owner_id, symbol, name, decimals, supply, status, created_at, updated_at, updated_event_sequence
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
            WHERE currencies.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(input.symbol.value())
        .bind(input.name.value())
        .bind(i16::from(input.decimals.value()))
        .bind(input.supply.value().to_string())
        .bind(Self::status_name(input.status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        owner: CurrencyOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn increase_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn decrease_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        status: CurrencyStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currencies
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
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyProjectionStoreError> {
        sqlx::query(
            r#"
            DELETE FROM currencies
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
