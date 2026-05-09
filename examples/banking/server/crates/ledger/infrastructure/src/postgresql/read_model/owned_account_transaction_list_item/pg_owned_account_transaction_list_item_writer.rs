use appletheia::application::event::EventSequence;
use appletheia::application::request_context::CorrelationId;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_application::{
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus, OwnedAccountTransactionListItemWriter,
    OwnedAccountTransactionListItemWriterError,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed owned account transaction list item writer.
pub struct PgOwnedAccountTransactionListItemWriter;

impl PgOwnedAccountTransactionListItemWriter {
    pub fn new() -> Self {
        Self
    }

    fn direction_name(direction: OwnedAccountTransactionListItemDirection) -> &'static str {
        match direction {
            OwnedAccountTransactionListItemDirection::Incoming => "incoming",
            OwnedAccountTransactionListItemDirection::Outgoing => "outgoing",
        }
    }

    fn kind_name(kind: OwnedAccountTransactionListItemKind) -> &'static str {
        match kind {
            OwnedAccountTransactionListItemKind::Deposit => "deposit",
            OwnedAccountTransactionListItemKind::Withdrawal => "withdrawal",
            OwnedAccountTransactionListItemKind::Transfer { .. } => "transfer",
            OwnedAccountTransactionListItemKind::CurrencyIssuance => "currency_issuance",
        }
    }

    fn status_name(status: OwnedAccountTransactionListItemStatus) -> &'static str {
        match status {
            OwnedAccountTransactionListItemStatus::Pending => "pending",
            OwnedAccountTransactionListItemStatus::Completed => "completed",
            OwnedAccountTransactionListItemStatus::Failed => "failed",
            OwnedAccountTransactionListItemStatus::RequiresReview => "requires_review",
        }
    }

    fn transfer_failure_status(
        reason: TransferFailureReason,
    ) -> OwnedAccountTransactionListItemStatus {
        match reason {
            TransferFailureReason::FundsReserveRejected
            | TransferFailureReason::DepositRejected => {
                OwnedAccountTransactionListItemStatus::Failed
            }
            TransferFailureReason::ReservedFundsReleaseRejected
            | TransferFailureReason::ReservedFundsCommitRejected => {
                OwnedAccountTransactionListItemStatus::RequiresReview
            }
        }
    }
}

impl Default for PgOwnedAccountTransactionListItemWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedAccountTransactionListItemWriter for PgOwnedAccountTransactionListItemWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_item_currencies (
                id, symbol, name, decimals, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                symbol = EXCLUDED.symbol,
                name = EXCLUDED.name,
                decimals = EXCLUDED.decimals,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_item_currencies.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(symbol.value())
        .bind(name.value())
        .bind(i16::from(decimals.value()))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_currencies
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_currencies
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM owned_account_transaction_list_items
             WHERE currency_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM owned_account_transaction_list_item_currencies
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_item_owner_users (
                id, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_item_owner_users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_users
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_users
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_users
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM owned_account_transaction_list_item_owner_users
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
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
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_item_owner_organizations (
                id, handle, display_name, picture_type, picture_object_name, picture_external_url,
                updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_item_owner_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(display_name.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_organizations
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_organizations
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
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_item_owner_organizations
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM owned_account_transaction_list_item_owner_organizations
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        id: EventId,
        correlation_id: CorrelationId,
        account_id: AccountId,
        counterparty_account_id: Option<AccountId>,
        amount: CurrencyAmount,
        direction: OwnedAccountTransactionListItemDirection,
        kind: OwnedAccountTransactionListItemKind,
        status: OwnedAccountTransactionListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_items (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, updated_event_sequence
            )
            SELECT
                $1, NULL, a.owner_type, a.owner_id, a.id, $3, a.currency_id,
                $4, $5, $6, $7, $8, $8, $8, $9
              FROM owned_account_list_items a
             WHERE a.id = $2
               AND NOT EXISTS (
                   SELECT 1
                     FROM owned_account_transaction_list_transfers t
                    WHERE t.correlation_id = $10
                      AND t.to_account_id = $2
               )
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
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(account_id.value())
        .bind(counterparty_account_id.map(|id| id.value()))
        .bind(amount.value().to_string())
        .bind(Self::direction_name(direction))
        .bind(Self::kind_name(kind))
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .bind(correlation_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        correlation_id: CorrelationId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_transfers (
                id, correlation_id, from_account_id, to_account_id, currency_id, amount,
                updated_at, created_at, updated_event_sequence
            )
            SELECT $1, $2, a.id, $3, a.currency_id, $4, $5, $5, $6
              FROM owned_account_list_items a
             WHERE a.id = $7
            ON CONFLICT (id) DO UPDATE SET
                correlation_id = EXCLUDED.correlation_id,
                from_account_id = EXCLUDED.from_account_id,
                to_account_id = EXCLUDED.to_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_transfers.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(correlation_id.value())
        .bind(to_account_id.value())
        .bind(amount.value().to_string())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .bind(from_account_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_items (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, updated_event_sequence
            )
            SELECT
                $1, $1, a.owner_type, a.owner_id, a.id, $2, a.currency_id, $3,
                'outgoing', 'transfer', 'pending', $4, $4, $4, $5
              FROM owned_account_list_items a
             WHERE a.id = $6
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
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(to_account_id.value())
        .bind(amount.value().to_string())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .bind(from_account_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_items
               SET status = 'completed',
                   updated_at = $2,
                   updated_event_sequence = $3
             WHERE id = $1 AND updated_event_sequence < $3
            "#,
        )
        .bind(id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_items (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, updated_event_sequence
            )
            SELECT
                $1, t.id, a.owner_type, a.owner_id, a.id, t.from_account_id,
                t.currency_id, t.amount, 'incoming', 'transfer', 'completed',
                $2, $2, $2, $3
              FROM owned_account_transaction_list_transfers t
              INNER JOIN owned_account_list_items a ON a.id = t.to_account_id
             WHERE t.id = $4
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
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(event_id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .bind(id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        reason: TransferFailureReason,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        let status = Self::status_name(Self::transfer_failure_status(reason));
        sqlx::query(
            r#"
            UPDATE owned_account_transaction_list_items
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(status)
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn record_currency_issuance_issued(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        destination_account_id: AccountId,
        currency_id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_currency_issuances (
                id, destination_account_id, currency_id, amount,
                updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                destination_account_id = EXCLUDED.destination_account_id,
                currency_id = EXCLUDED.currency_id,
                amount = EXCLUDED.amount,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_currency_issuances.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(destination_account_id.value())
        .bind(currency_id.value())
        .bind(amount.value().to_string())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn complete_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO owned_account_transaction_list_items (
                id, transfer_id, owner_type, owner_id, account_id, counterparty_account_id,
                currency_id, amount, direction, kind, status, occurred_at, updated_at,
                created_at, updated_event_sequence
            )
            SELECT
                $1, NULL, a.owner_type, a.owner_id, a.id, NULL,
                i.currency_id, i.amount, 'incoming', 'currency_issuance', 'completed',
                $2, $2, $2, $3
              FROM owned_account_transaction_list_currency_issuances i
              INNER JOIN owned_account_list_items a ON a.id = i.destination_account_id
             WHERE i.id = $4
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
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE owned_account_transaction_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(event_id.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .bind(id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn fail_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM owned_account_transaction_list_currency_issuances
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OwnedAccountTransactionListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
