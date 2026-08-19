use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    AccountFragment, AccountFragmentUpsert, AccountFragmentWriter, AccountFragmentWriterError,
    MaterializedAccountStatus,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use uuid::Uuid;

use super::pg_account_fragment_row::PgAccountFragmentRow;

/// PostgreSQL-backed account fragment writer.
pub struct PgAccountFragmentWriter;

impl PgAccountFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: AccountOwner) -> (&'static str, Uuid) {
        match owner {
            AccountOwner::User(user_id) => ("user", user_id.value()),
            AccountOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: MaterializedAccountStatus) -> &'static str {
        match status {
            MaterializedAccountStatus::Active => "active",
            MaterializedAccountStatus::Frozen => "frozen",
        }
    }

    async fn load_changed_fragment(
        uow: &mut PgUnitOfWork,
        id: AccountId,
        rows_affected: u64,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        if rows_affected == 0 {
            return Ok(None);
        }

        let row = sqlx::query_as::<_, PgAccountFragmentRow>(
            r#"
            SELECT id, owner_type, owner_id, name, currency_id,
                   balance::text AS balance, reserved_balance::text AS reserved_balance,
                   status, created_at, source_event_id, updated_event_id
              FROM account_fragments
             WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        row.map(AccountFragment::try_from).transpose()
    }
}

impl Default for PgAccountFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountFragmentWriter for PgAccountFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: AccountFragmentUpsert,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(upsert.owner);

        let result = sqlx::query(
            r#"
            INSERT INTO account_fragments (
                id, owner_type, owner_id, name, currency_id, balance, reserved_balance, status, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11, $12, $12)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                name = EXCLUDED.name,
                currency_id = EXCLUDED.currency_id,
                balance = EXCLUDED.balance,
                reserved_balance = EXCLUDED.reserved_balance,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(upsert.name.value())
        .bind(upsert.currency_id.value())
        .bind(upsert.balance.value().to_string())
        .bind(upsert.reserved_balance.value().to_string())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, upsert.id, result.rows_affected()).await
    }

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        owner: AccountOwner,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET owner_type = $2, owner_id = $3, updated_at = $4,
                   updated_event_sequence = $5,
                   updated_event_id = $6
             WHERE id = $1 AND updated_event_sequence < $5
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_account_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        name: AccountName,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET name = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET balance = balance + $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET balance = balance - $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn reserve_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET balance = balance - $2,
                   reserved_balance = reserved_balance + $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn release_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET balance = balance + $2,
                   reserved_balance = reserved_balance - $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn commit_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET reserved_balance = reserved_balance - $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        status: MaterializedAccountStatus,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_fragments
               SET status = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
    ) -> Result<bool, AccountFragmentWriterError> {
        let result = sqlx::query(
            r#"
            DELETE FROM account_fragments
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(result.rows_affected() > 0)
    }
}
