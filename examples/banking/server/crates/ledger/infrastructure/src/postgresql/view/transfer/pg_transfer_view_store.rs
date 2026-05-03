use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{TransferViewStore, TransferViewStoreError, TransferViewUpsert};
use banking_ledger_domain::transfer::{TransferId, TransferStatus};

/// PostgreSQL-backed transfer view store.
pub struct PgTransferViewStore;

impl PgTransferViewStore {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: TransferStatus) -> &'static str {
        match status {
            TransferStatus::Pending => "pending",
            TransferStatus::Completed => "completed",
            TransferStatus::Failed => "failed",
            TransferStatus::Cancelled => "cancelled",
            TransferStatus::Rejected => "rejected",
        }
    }
}

impl Default for PgTransferViewStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferViewStore for PgTransferViewStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: TransferViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), TransferViewStoreError> {
        sqlx::query(
            r#"
            INSERT INTO transfers (
                id, from_account_id, to_account_id, amount, status, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                from_account_id = EXCLUDED.from_account_id,
                to_account_id = EXCLUDED.to_account_id,
                amount = EXCLUDED.amount,
                status = EXCLUDED.status,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE transfers.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.from_account_id.value())
        .bind(input.to_account_id.value())
        .bind(input.amount.value().to_string())
        .bind(Self::status_name(input.status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| TransferViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        status: TransferStatus,
        event_sequence: EventSequence,
    ) -> Result<(), TransferViewStoreError> {
        sqlx::query(
            r#"
            UPDATE transfers
               SET status = $2, updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| TransferViewStoreError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
