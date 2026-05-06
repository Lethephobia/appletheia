use appletheia::application::event::EventSequence;
use appletheia::domain::{AggregateId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    PublicAccountListItemStatus, PublicAccountListItemWriter, PublicAccountListItemWriterError,
};
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

/// PostgreSQL-backed public account list item writer.
pub struct PgPublicAccountListItemWriter;

impl PgPublicAccountListItemWriter {
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

    fn status_name(status: PublicAccountListItemStatus) -> &'static str {
        match status {
            PublicAccountListItemStatus::Active => "active",
            PublicAccountListItemStatus::Frozen => "frozen",
        }
    }
}

impl Default for PgPublicAccountListItemWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicAccountListItemWriter for PgPublicAccountListItemWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        currency_id: CurrencyId,
        status: PublicAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            INSERT INTO public_account_list_item_accounts (
                id, owner_type, owner_id, currency_id, status, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                currency_id = EXCLUDED.currency_id,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_item_accounts.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(currency_id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            UPDATE public_account_list_item_accounts
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
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: PublicAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_accounts
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
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        event_sequence: EventSequence,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_accounts
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            INSERT INTO public_account_list_item_currencies (
                id, symbol, name, decimals, updated_at, created_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                symbol = EXCLUDED.symbol,
                name = EXCLUDED.name,
                decimals = EXCLUDED.decimals,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_item_currencies.updated_event_sequence < EXCLUDED.updated_event_sequence
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
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_currencies
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
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_currencies
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
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
    ) -> Result<(), PublicAccountListItemWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_accounts
             WHERE currency_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_currencies
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListItemWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
