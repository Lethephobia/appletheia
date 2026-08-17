use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    WalletBookmarkFragment, WalletBookmarkFragmentUpsert, WalletBookmarkFragmentWriter,
    WalletBookmarkFragmentWriterError,
};
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};
use uuid::Uuid;

use super::super::PgFragmentLoader;
use super::pg_wallet_bookmark_fragment_row::PgWalletBookmarkFragmentRow;

/// PostgreSQL-backed wallet bookmark fragment writer.
pub struct PgWalletBookmarkFragmentWriter;

impl PgWalletBookmarkFragmentWriter {
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

    async fn load_changed_fragment(
        uow: &mut PgUnitOfWork,
        id: WalletBookmarkId,
        rows_affected: u64,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError> {
        if rows_affected == 0 {
            return Ok(None);
        }

        let row = sqlx::query_as::<_, PgWalletBookmarkFragmentRow>(
            r#"
            SELECT id AS wallet_bookmark_id, owner_type, owner_id, display_name,
                   description, token_account_owner_address, created_at,
                   source_event_id, updated_event_id
              FROM wallet_bookmark_fragments
             WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| WalletBookmarkFragmentWriterError::Persistence(Box::new(error)))?;

        let Some(wallet_bookmark_row) = row else {
            return Ok(None);
        };
        let owner = PgFragmentLoader::load_owner(
            uow,
            &wallet_bookmark_row.owner_type,
            wallet_bookmark_row.owner_id,
        )
        .await
        .map_err(WalletBookmarkFragmentWriterError::Persistence)?;

        wallet_bookmark_row.try_into_fragment(owner).map(Some)
    }
}

impl Default for PgWalletBookmarkFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletBookmarkFragmentWriter for PgWalletBookmarkFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: WalletBookmarkFragmentUpsert,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(upsert.owner);

        let result = sqlx::query(
            r#"
            INSERT INTO wallet_bookmark_fragments (
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
            WHERE wallet_bookmark_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
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
        .map_err(|error| WalletBookmarkFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, upsert.id, result.rows_affected()).await
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE wallet_bookmark_fragments
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
        .map_err(|error| WalletBookmarkFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<Option<WalletBookmarkFragment>, WalletBookmarkFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE wallet_bookmark_fragments
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
        .map_err(|error| WalletBookmarkFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn delete_wallet_bookmark(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: WalletBookmarkId,
    ) -> Result<bool, WalletBookmarkFragmentWriterError> {
        let result = sqlx::query(
            r#"
            DELETE FROM wallet_bookmark_fragments
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| WalletBookmarkFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(result.rows_affected() > 0)
    }
}
