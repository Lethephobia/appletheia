use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    AccountTransactionCurrencyIssuanceIssuedRecord, AccountTransactionDirection,
    AccountTransactionFragment, AccountTransactionFragmentInsert, AccountTransactionFragmentKind,
    AccountTransactionFragmentWriter, AccountTransactionFragmentWriterError, AccountTransactionId,
    AccountTransactionStatus, AccountTransactionTransferRequestedRecord,
};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};

use super::pg_account_transaction_fragment_row::PgAccountTransactionFragmentRow;

/// PostgreSQL-backed account transaction fragment writer.
pub struct PgAccountTransactionFragmentWriter;

impl PgAccountTransactionFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn direction_name(direction: AccountTransactionDirection) -> &'static str {
        match direction {
            AccountTransactionDirection::Incoming => "incoming",
            AccountTransactionDirection::Outgoing => "outgoing",
        }
    }

    fn kind_name(kind: AccountTransactionFragmentKind) -> &'static str {
        match kind {
            AccountTransactionFragmentKind::Deposit => "deposit",
            AccountTransactionFragmentKind::Withdrawal => "withdrawal",
            AccountTransactionFragmentKind::Transfer => "transfer",
            AccountTransactionFragmentKind::CurrencyIssuance => "currency_issuance",
        }
    }

    fn status_name(status: AccountTransactionStatus) -> &'static str {
        match status {
            AccountTransactionStatus::Pending => "pending",
            AccountTransactionStatus::Completed => "completed",
            AccountTransactionStatus::Failed => "failed",
            AccountTransactionStatus::RequiresReview => "requires_review",
        }
    }

    fn transfer_failure_status(reason: TransferFailureReason) -> AccountTransactionStatus {
        match reason {
            TransferFailureReason::FundsReserveRejected
            | TransferFailureReason::DepositRejected => AccountTransactionStatus::Failed,
            TransferFailureReason::ReservedFundsReleaseRejected
            | TransferFailureReason::ReservedFundsCommitRejected => {
                AccountTransactionStatus::RequiresReview
            }
        }
    }

    async fn load_changed_fragment(
        uow: &mut PgUnitOfWork,
        id: AccountTransactionId,
        rows_affected: u64,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        if rows_affected == 0 {
            return Ok(None);
        }

        let row = sqlx::query_as::<_, PgAccountTransactionFragmentRow>(
            r#"
            SELECT id AS transaction_id, transfer_id, account_id,
                   counterparty_account_id, amount::text AS amount,
                   direction, kind, status, occurred_at, created_at,
                   source_event_id, updated_event_id
              FROM account_transaction_fragments
             WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        row.map(AccountTransactionFragment::try_from).transpose()
    }
}

impl Default for PgAccountTransactionFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountTransactionFragmentWriter for PgAccountTransactionFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        insert: AccountTransactionFragmentInsert,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_transaction_fragments (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT
                $1, NULL, a.owner_type, a.owner_id, a.id, $3, a.currency_id,
                $4, $5, $6, $7, $8, $8, $8, $9, $9, $10, $10
              FROM account_fragments a
             WHERE a.id = $2
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                account_id = EXCLUDED.account_id,
                counterparty_account_id = EXCLUDED.counterparty_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                direction = EXCLUDED.direction,
                kind = EXCLUDED.kind,
                status = EXCLUDED.status,
                occurred_at = EXCLUDED.occurred_at,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(insert.transaction_id.value())
        .bind(insert.account_id.value())
        .bind(insert.counterparty_account_id.map(|id| id.value()))
        .bind(insert.amount.value().to_string())
        .bind(Self::direction_name(insert.direction))
        .bind(Self::kind_name(insert.kind))
        .bind(Self::status_name(insert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, insert.transaction_id, result.rows_affected()).await
    }

    async fn update_account_transaction_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountTransactionId,
        status: AccountTransactionStatus,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE account_transaction_fragments
               SET status = $2,
                   updated_at = $3,
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
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        record: AccountTransactionTransferRequestedRecord,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        sqlx::query(
            r#"
            INSERT INTO account_transaction_transfer_fragments (
                id, correlation_id, from_account_id, to_account_id, currency_id, amount,
                updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT $1, $2, a.id, $3, a.currency_id, $4, $5, $5, $6, $6, $7, $7
              FROM account_fragments a
             WHERE a.id = $8
            ON CONFLICT (id) DO UPDATE SET
                correlation_id = EXCLUDED.correlation_id,
                from_account_id = EXCLUDED.from_account_id,
                to_account_id = EXCLUDED.to_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_transfer_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(record.id.value())
        .bind(record.correlation_id.value())
        .bind(record.to_account_id.value())
        .bind(record.amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(record.from_account_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountTransactionFragmentWriterError::Persistence(Box::new(e)))?;

        let transaction_result = sqlx::query(
            r#"
            INSERT INTO account_transaction_fragments (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT
                $1, $1, a.owner_type, a.owner_id, a.id, $2, a.currency_id, $3,
                'outgoing', 'transfer', 'pending', $4, $4, $4, $5, $5, $6, $6
              FROM account_fragments a
             WHERE a.id = $7
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                account_id = EXCLUDED.account_id,
                counterparty_account_id = EXCLUDED.counterparty_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                direction = EXCLUDED.direction,
                kind = EXCLUDED.kind,
                status = EXCLUDED.status,
                occurred_at = EXCLUDED.occurred_at,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(record.id.value())
        .bind(record.to_account_id.value())
        .bind(record.amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(record.from_account_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(
            uow,
            AccountTransactionId::from(record.id.value()),
            transaction_result.rows_affected(),
        )
        .await
    }

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: TransferId,
        transaction_id: AccountTransactionId,
    ) -> Result<Vec<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        let outgoing_result = sqlx::query(
            r#"
            UPDATE account_transaction_fragments
               SET status = 'completed',
                   updated_at = $2,
                   updated_event_sequence = $3,
                   updated_event_id = $4
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        let incoming_result = sqlx::query(
            r#"
            INSERT INTO account_transaction_fragments (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT
                $1, t.id, a.owner_type, a.owner_id, a.id, t.from_account_id,
                t.currency_id, t.amount, 'incoming', 'transfer', 'completed',
                $2, $2, $2, $3, $3, $4, $4
              FROM account_transaction_transfer_fragments t
              INNER JOIN account_fragments a ON a.id = t.to_account_id
             WHERE t.id = $5
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                account_id = EXCLUDED.account_id,
                counterparty_account_id = EXCLUDED.counterparty_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                direction = EXCLUDED.direction,
                kind = EXCLUDED.kind,
                status = EXCLUDED.status,
                occurred_at = EXCLUDED.occurred_at,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(transaction_id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        let mut fragments = Vec::new();
        if let Some(fragment) = Self::load_changed_fragment(
            uow,
            AccountTransactionId::from(id.value()),
            outgoing_result.rows_affected(),
        )
        .await?
        {
            fragments.push(fragment);
        }
        if let Some(fragment) =
            Self::load_changed_fragment(uow, transaction_id, incoming_result.rows_affected())
                .await?
        {
            fragments.push(fragment);
        }

        Ok(fragments)
    }

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: TransferId,
        reason: TransferFailureReason,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        let status = Self::status_name(Self::transfer_failure_status(reason));
        let result = sqlx::query(
            r#"
            UPDATE account_transaction_fragments
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(status)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(
            uow,
            AccountTransactionId::from(id.value()),
            result.rows_affected(),
        )
        .await
    }

    async fn record_currency_issuance_issued(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        record: AccountTransactionCurrencyIssuanceIssuedRecord,
    ) -> Result<(), AccountTransactionFragmentWriterError> {
        sqlx::query(
            r#"
            INSERT INTO account_transaction_currency_issuance_fragments (
                id, destination_account_id, currency_id, amount,
                updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6, $6, $7, $7)
            ON CONFLICT (id) DO UPDATE SET
                destination_account_id = EXCLUDED.destination_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_currency_issuance_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(record.id.value())
        .bind(record.destination_account_id.value())
        .bind(record.currency_id.value())
        .bind(record.amount.value().to_string())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(())
    }

    async fn complete_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyIssuanceId,
        transaction_id: AccountTransactionId,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_transaction_fragments (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            SELECT
                $1, NULL, a.owner_type, a.owner_id, a.id, NULL,
                i.currency_id, i.amount, 'incoming', 'currency_issuance', 'completed',
                $2, $2, $2, $3, $3, $4, $4
              FROM account_transaction_currency_issuance_fragments i
              INNER JOIN account_fragments a ON a.id = i.destination_account_id
             WHERE i.id = $5
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                account_id = EXCLUDED.account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                direction = EXCLUDED.direction,
                kind = EXCLUDED.kind,
                status = EXCLUDED.status,
                occurred_at = EXCLUDED.occurred_at,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE account_transaction_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(transaction_id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| AccountTransactionFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, transaction_id, result.rows_affected()).await
    }

    async fn fail_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyIssuanceId,
    ) -> Result<(), AccountTransactionFragmentWriterError> {
        sqlx::query(
            r#"
            DELETE FROM account_transaction_currency_issuance_fragments
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| AccountTransactionFragmentWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
