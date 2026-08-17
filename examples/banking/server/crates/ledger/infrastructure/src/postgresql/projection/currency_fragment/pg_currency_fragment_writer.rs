use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyFragment, CurrencyFragmentUpsert, CurrencyFragmentWriter, CurrencyFragmentWriterError,
    MaterializedCurrencyStatus,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName, CurrencyOwner, CurrencySymbol,
    MintAccountAddress,
};
use uuid::Uuid;

use crate::postgresql::pg_currency_image_ref_columns::PgCurrencyImageRefColumns;

use super::super::PgFragmentLoader;
use super::pg_currency_fragment_row::PgCurrencyFragmentRow;

/// PostgreSQL-backed currency fragment writer.
pub struct PgCurrencyFragmentWriter;

impl PgCurrencyFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: CurrencyOwner) -> (&'static str, Uuid) {
        match owner {
            CurrencyOwner::User(user_id) => ("user", user_id.value()),
            CurrencyOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: MaterializedCurrencyStatus) -> &'static str {
        match status {
            MaterializedCurrencyStatus::Provisioning => "provisioning",
            MaterializedCurrencyStatus::Active => "active",
            MaterializedCurrencyStatus::Inactive => "inactive",
            MaterializedCurrencyStatus::ProvisioningFailed => "provisioning_failed",
        }
    }

    async fn load_changed_fragment(
        uow: &mut PgUnitOfWork,
        id: CurrencyId,
        rows_affected: u64,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        if rows_affected == 0 {
            return Ok(None);
        }

        let row = sqlx::query_as::<_, PgCurrencyFragmentRow>(
            r#"
            SELECT id, owner_type, owner_id, symbol, name, decimals,
                   description, image_type, image_object_name, image_external_url,
                   mint_account_address, supply::text AS supply, status, created_at,
                   source_event_id, updated_event_id
              FROM currency_fragments
             WHERE id = $1
            "#,
        )
        .bind(id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        let Some(currency_row) = row else {
            return Ok(None);
        };
        let owner =
            PgFragmentLoader::load_owner(uow, &currency_row.owner_type, currency_row.owner_id)
                .await
                .map_err(CurrencyFragmentWriterError::Persistence)?;

        currency_row.try_into_fragment(owner).map(Some)
    }
}

impl Default for PgCurrencyFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyFragmentWriter for PgCurrencyFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: CurrencyFragmentUpsert,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(upsert.owner);
        let (image_type, image_object_name, image_external_url) =
            PgCurrencyImageRefColumns::from_image(upsert.image.as_ref());

        let result = sqlx::query(
            r#"
            INSERT INTO currency_fragments (
                id, owner_type, owner_id, symbol, name, decimals, description, image_type,
                image_object_name, image_external_url, mint_account_address, supply, status, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $16, $17, $17)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                symbol = EXCLUDED.symbol,
                name = EXCLUDED.name,
                decimals = EXCLUDED.decimals,
                description = EXCLUDED.description,
                image_type = EXCLUDED.image_type,
                image_object_name = EXCLUDED.image_object_name,
                image_external_url = EXCLUDED.image_external_url,
                mint_account_address = EXCLUDED.mint_account_address,
                supply = EXCLUDED.supply,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE currency_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(upsert.symbol.value())
        .bind(upsert.name.value())
        .bind(i16::from(upsert.decimals.value()))
        .bind(upsert.description.as_ref().map(CurrencyDescription::value))
        .bind(image_type)
        .bind(image_object_name)
        .bind(image_external_url)
        .bind(
            upsert
                .mint_account_address
                .as_ref()
                .map(MintAccountAddress::value),
        )
        .bind(upsert.supply.value().to_string())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, upsert.id, result.rows_affected()).await
    }

    async fn update_currency_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
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
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
               SET symbol = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(symbol.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
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
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_currency_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        description: Option<CurrencyDescription>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
               SET description = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(description.as_ref().map(CurrencyDescription::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_currency_image(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        image: Option<CurrencyImageRef>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let (image_type, image_object_name, image_external_url) =
            PgCurrencyImageRefColumns::from_image(image.as_ref());

        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
               SET image_type = $2,
                   image_object_name = $3,
                   image_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6,
                   updated_event_id = $7
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(image_type)
        .bind(image_object_name)
        .bind(image_external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn provision_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        mint_account_address: MintAccountAddress,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
               SET mint_account_address = $2,
                   status = 'active',
                   updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(mint_account_address.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn increase_currency_supply(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        amount: CurrencyAmount,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
               SET supply = supply + $2,
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
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        status: MaterializedCurrencyStatus,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            UPDATE currency_fragments
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
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Self::load_changed_fragment(uow, id, result.rows_affected()).await
    }

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
    ) -> Result<bool, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"
            DELETE FROM currency_fragments
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| CurrencyFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(result.rows_affected() > 0)
    }
}
