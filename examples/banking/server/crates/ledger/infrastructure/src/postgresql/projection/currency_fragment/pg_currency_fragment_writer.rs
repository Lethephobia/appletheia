use appletheia::application::read_model::{MaterializationEventContext, ReadModelObservation};
use appletheia::domain::{AggregateId, EventId};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyFragment, CurrencyFragmentUpsert, CurrencyFragmentWriter, CurrencyFragmentWriterError,
    CurrencyTokenBindingFragment,
};
use banking_ledger_domain::core::{ChainNetwork, CurrencyCode, CurrencyDecimals, TokenAddress};
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId, CurrencyStatus};
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use banking_ledger_domain::token_binding::TokenBindingId;
use uuid::Uuid;

use super::PgCurrencyFragmentWriterError;

pub struct PgCurrencyFragmentWriter;

impl PgCurrencyFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    async fn load(
        uow: &mut PgUnitOfWork,
        id: CurrencyId,
        rows_affected: u64,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        if rows_affected == 0 {
            return Ok(None);
        }

        #[derive(sqlx::FromRow)]
        struct CurrencyRow {
            id: Uuid,
            currency_registrar_id: Uuid,
            code: String,
            decimals: i16,
            description: Option<String>,
            status: String,
            source_event_id: Uuid,
            updated_event_id: Uuid,
        }
        #[derive(sqlx::FromRow)]
        struct BindingRow {
            id: Uuid,
            chain_network: String,
            token_address: String,
        }

        let row = sqlx::query_as::<_, CurrencyRow>(
            "SELECT id, currency_registrar_id, code, decimals, description, status, source_event_id, updated_event_id FROM currency_fragments WHERE id = $1",
        )
        .bind(id.value())
        .fetch_one(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        let binding_rows = sqlx::query_as::<_, BindingRow>(
            "SELECT id, chain_network, token_address FROM currency_token_binding_fragments WHERE currency_id = $1 ORDER BY id",
        )
        .bind(id.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;

        let status = match row.status.as_str() {
            "defined" => CurrencyStatus::Defined,
            "active" => CurrencyStatus::Active,
            "inactive" => CurrencyStatus::Inactive,
            _ => {
                return Err(persistence(PgCurrencyFragmentWriterError::Status(
                    row.status.clone(),
                )));
            }
        };
        let token_bindings = binding_rows
            .into_iter()
            .map(|binding| {
                Ok(CurrencyTokenBindingFragment {
                    id: TokenBindingId::try_from_uuid(binding.id).map_err(persistence)?,
                    chain_network: serde_json::from_str::<ChainNetwork>(&binding.chain_network)
                        .map_err(persistence)?,
                    token_address: serde_json::from_str::<TokenAddress>(&binding.token_address)
                        .map_err(persistence)?,
                })
            })
            .collect::<Result<Vec<_>, CurrencyFragmentWriterError>>()?;

        Ok(Some(CurrencyFragment {
            id: CurrencyId::try_from_uuid(row.id).map_err(persistence)?,
            currency_registrar_id: CurrencyRegistrarId::try_from_uuid(row.currency_registrar_id)
                .map_err(persistence)?,
            code: CurrencyCode::try_from(row.code).map_err(persistence)?,
            decimals: CurrencyDecimals::new(u8::try_from(row.decimals).map_err(persistence)?),
            description: row
                .description
                .map(CurrencyDescription::try_from)
                .transpose()
                .map_err(persistence)?,
            status,
            token_bindings,
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence)?,
                EventId::try_from(row.updated_event_id).map_err(persistence)?,
            ),
        }))
    }

    const fn currency_status(status: CurrencyStatus) -> &'static str {
        match status {
            CurrencyStatus::Defined => "defined",
            CurrencyStatus::Active => "active",
            CurrencyStatus::Inactive => "inactive",
        }
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
        context: MaterializationEventContext,
        upsert: CurrencyFragmentUpsert,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            r#"INSERT INTO currency_fragments
               (id, currency_registrar_id, code, decimals, description, status, created_at, updated_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8, $9, $9)
               ON CONFLICT (id) DO UPDATE SET
                 currency_registrar_id = EXCLUDED.currency_registrar_id,
                 code = EXCLUDED.code, decimals = EXCLUDED.decimals,
                 description = EXCLUDED.description,
                 status = EXCLUDED.status, updated_at = EXCLUDED.updated_at,
                 updated_event_sequence = EXCLUDED.updated_event_sequence,
                 updated_event_id = EXCLUDED.updated_event_id
               WHERE currency_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence"#,
        )
        .bind(upsert.id.value())
        .bind(upsert.currency_registrar_id.value())
        .bind(upsert.code.value())
        .bind(i16::from(upsert.decimals.value()))
        .bind(upsert.description.as_ref().map(AsRef::<str>::as_ref))
        .bind(Self::currency_status(upsert.status))
        .bind(context.occurred_at.value())
        .bind(context.event_sequence.value())
        .bind(context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        Self::load(uow, upsert.id, result.rows_affected()).await
    }

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        context: MaterializationEventContext,
        id: CurrencyId,
        status: CurrencyStatus,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            "UPDATE currency_fragments SET status = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5 WHERE id = $1 AND updated_event_sequence < $4",
        )
        .bind(id.value())
        .bind(Self::currency_status(status))
        .bind(context.occurred_at.value())
        .bind(context.event_sequence.value())
        .bind(context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        Self::load(uow, id, result.rows_affected()).await
    }

    async fn update_currency_description(
        &self,
        uow: &mut Self::Uow,
        context: MaterializationEventContext,
        id: CurrencyId,
        description: Option<CurrencyDescription>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            "UPDATE currency_fragments SET description = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5 WHERE id = $1 AND updated_event_sequence < $4",
        )
        .bind(id.value())
        .bind(description.as_ref().map(AsRef::<str>::as_ref))
        .bind(context.occurred_at.value())
        .bind(context.event_sequence.value())
        .bind(context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        Self::load(uow, id, result.rows_affected()).await
    }

    async fn define_token_binding(
        &self,
        uow: &mut Self::Uow,
        context: MaterializationEventContext,
        currency_id: CurrencyId,
        token_binding_id: TokenBindingId,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let result = sqlx::query(
            "UPDATE currency_fragments SET updated_at = $2, updated_event_sequence = $3, updated_event_id = $4 WHERE id = $1 AND updated_event_sequence < $3",
        )
        .bind(currency_id.value())
        .bind(context.occurred_at.value())
        .bind(context.event_sequence.value())
        .bind(context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        if result.rows_affected() != 0 {
            sqlx::query(
                r#"INSERT INTO currency_token_binding_fragments
                   (id, currency_id, chain_network, token_address, created_at, updated_at,
                    source_event_sequence, updated_event_sequence, source_event_id, updated_event_id)
                   VALUES ($1, $2, $3, $4, $5, $5, $6, $6, $7, $7)"#,
            )
            .bind(token_binding_id.value())
            .bind(currency_id.value())
            .bind(serde_json::to_string(&chain_network).map_err(persistence)?)
            .bind(serde_json::to_string(&token_address).map_err(persistence)?)
            .bind(context.occurred_at.value())
            .bind(context.event_sequence.value())
            .bind(context.event_id.value())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(persistence)?;
        }
        Self::load(uow, currency_id, result.rows_affected()).await
    }

    async fn remove_token_binding(
        &self,
        uow: &mut Self::Uow,
        context: MaterializationEventContext,
        token_binding_id: TokenBindingId,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError> {
        let currency_id_lookup = sqlx::query_scalar::<_, Uuid>(
            "SELECT currency_id FROM currency_token_binding_fragments WHERE id = $1",
        )
        .bind(token_binding_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        let Some(raw_currency_id) = currency_id_lookup else {
            return Ok(None);
        };
        let currency_id = CurrencyId::try_from_uuid(raw_currency_id).map_err(persistence)?;
        let result = sqlx::query(
            "UPDATE currency_fragments SET updated_at = $2, updated_event_sequence = $3, updated_event_id = $4 WHERE id = $1 AND updated_event_sequence < $3",
        )
        .bind(currency_id.value())
        .bind(context.occurred_at.value())
        .bind(context.event_sequence.value())
        .bind(context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        if result.rows_affected() != 0 {
            sqlx::query("DELETE FROM currency_token_binding_fragments WHERE id = $1")
                .bind(token_binding_id.value())
                .execute(uow.transaction_mut().as_mut())
                .await
                .map_err(persistence)?;
        }
        Self::load(uow, currency_id, result.rows_affected()).await
    }
}

fn persistence(
    error: impl std::error::Error + Send + Sync + 'static,
) -> CurrencyFragmentWriterError {
    CurrencyFragmentWriterError::Persistence(Box::new(error))
}
