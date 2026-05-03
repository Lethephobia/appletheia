use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyIssuanceProjectionStore, CurrencyIssuanceProjectionStoreError,
    CurrencyIssuanceProjectionUpsert,
};
use banking_ledger_domain::currency_issuance::{CurrencyIssuanceId, CurrencyIssuanceStatus};

/// PostgreSQL-backed currency issuance projection store.
pub struct PgCurrencyIssuanceProjectionStore;

impl PgCurrencyIssuanceProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: CurrencyIssuanceStatus) -> &'static str {
        match status {
            CurrencyIssuanceStatus::Pending => "pending",
            CurrencyIssuanceStatus::Completed => "completed",
            CurrencyIssuanceStatus::Failed => "failed",
            CurrencyIssuanceStatus::Rejected => "rejected",
        }
    }
}

impl Default for PgCurrencyIssuanceProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyIssuanceProjectionStore for PgCurrencyIssuanceProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: CurrencyIssuanceProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyIssuanceProjectionStoreError> {
        sqlx::query(
            r#"
            INSERT INTO currency_issuances (
                id, currency_id, destination_account_id, amount, status, created_at, updated_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                currency_id = EXCLUDED.currency_id,
                destination_account_id = EXCLUDED.destination_account_id,
                amount = EXCLUDED.amount,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE currency_issuances.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.currency_id.value())
        .bind(input.destination_account_id.value())
        .bind(input.amount.value().to_string())
        .bind(Self::status_name(input.status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| CurrencyIssuanceProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        status: CurrencyIssuanceStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyIssuanceProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE currency_issuances
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
        .map_err(|e| CurrencyIssuanceProjectionStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
