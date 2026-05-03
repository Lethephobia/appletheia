use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{AccountViewStore, AccountViewStoreError, AccountViewUpsert};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;

/// PostgreSQL-backed account view store.
pub struct PgAccountViewStore;

impl PgAccountViewStore {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: AccountOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            AccountOwner::User(user_id) => ("user", user_id.value()),
            AccountOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: AccountStatus) -> &'static str {
        match status {
            AccountStatus::Active => "active",
            AccountStatus::Frozen => "frozen",
            AccountStatus::Closed => "closed",
        }
    }
}

impl Default for PgAccountViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountViewStore for PgAccountViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: AccountViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(input.owner);

        sqlx::query(
            r#"
            INSERT INTO accounts (
                id, owner_type, owner_id, name, currency_id, balance, reserved_balance, status, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                name = EXCLUDED.name,
                currency_id = EXCLUDED.currency_id,
                balance = EXCLUDED.balance,
                reserved_balance = EXCLUDED.reserved_balance,
                status = EXCLUDED.status,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE accounts.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(input.name.value())
        .bind(input.currency_id.value())
        .bind(input.balance.value().to_string())
        .bind(input.reserved_balance.value().to_string())
        .bind(Self::status_name(input.status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        sqlx::query(
            r#"
            UPDATE accounts
               SET owner_type = $2, owner_id = $3, updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_name(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        name: AccountName,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET name = $2, updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(name.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET balance = balance + $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET balance = balance - $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn move_balance_to_reserved(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET balance = balance - $2,
                   reserved_balance = reserved_balance + $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn move_reserved_to_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET balance = balance + $2,
                   reserved_balance = reserved_balance - $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn decrease_reserved(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET reserved_balance = reserved_balance - $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: AccountStatus,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError> {
        sqlx::query(
            r#"
            UPDATE accounts
               SET status = $2, updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
