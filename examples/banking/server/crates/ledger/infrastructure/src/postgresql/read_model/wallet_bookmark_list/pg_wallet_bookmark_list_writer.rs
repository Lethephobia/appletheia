use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    WalletBookmarkListUpsert, WalletBookmarkListWriter, WalletBookmarkListWriterError,
};
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;
use uuid::Uuid;

/// PostgreSQL-backed wallet bookmark list writer.
pub struct PgWalletBookmarkListWriter;

impl PgWalletBookmarkListWriter {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: WalletBookmarkOwner) -> (&'static str, Uuid) {
        match owner {
            WalletBookmarkOwner::User(user_id) => ("user", user_id.value()),
            WalletBookmarkOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }
}

impl Default for PgWalletBookmarkListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletBookmarkListWriter for PgWalletBookmarkListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: WalletBookmarkListUpsert,
    ) -> Result<(), WalletBookmarkListWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(upsert.owner);

        sqlx::query(
            r#"
            INSERT INTO wallet_bookmark_list_items (
                id, owner_type, owner_id, display_name, description, token_account_owner_address,
                updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id,
                updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $10, $10)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                display_name = EXCLUDED.display_name,
                description = EXCLUDED.description,
                token_account_owner_address = EXCLUDED.token_account_owner_address,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE wallet_bookmark_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(upsert.display_name.as_ref().map(WalletBookmarkDisplayName::value))
        .bind(upsert.description.as_ref().map(WalletBookmarkDescription::value))
        .bind(upsert.token_account_owner_address.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| WalletBookmarkListWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<(), WalletBookmarkListWriterError> {
        sqlx::query(
            r#"
            UPDATE wallet_bookmark_list_items
               SET display_name = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.as_ref().map(WalletBookmarkDisplayName::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| WalletBookmarkListWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<(), WalletBookmarkListWriterError> {
        sqlx::query(
            r#"
            UPDATE wallet_bookmark_list_items
               SET description = $2,
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(WalletBookmarkDescription::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| WalletBookmarkListWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: WalletBookmarkId,
    ) -> Result<(), WalletBookmarkListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM wallet_bookmark_list_items
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| WalletBookmarkListWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
