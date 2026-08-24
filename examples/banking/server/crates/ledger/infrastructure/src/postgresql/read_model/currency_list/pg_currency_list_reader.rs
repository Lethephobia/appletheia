use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_ledger_application::{
    CurrencyFragment, CurrencyList, CurrencyListReader, CurrencyListReaderError,
    CurrencyTokenBindingFragment,
};
use banking_ledger_domain::core::{ChainNetwork, CurrencyCode, CurrencyDecimals, TokenAddress};
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId, CurrencyStatus};
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use banking_ledger_domain::token_binding::TokenBindingId;
use uuid::Uuid;

use super::PgCurrencyListReaderError;

pub struct PgCurrencyListReader;

impl PgCurrencyListReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgCurrencyListReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrencyListReader for PgCurrencyListReader {
    type Uow = PgUnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        include_inactive: bool,
    ) -> Result<CurrencyList, CurrencyListReaderError> {
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
            currency_id: Uuid,
            chain_network: String,
            token_address: String,
        }

        let rows = sqlx::query_as::<_, CurrencyRow>(
            "SELECT id, currency_registrar_id, code, decimals, description, status, source_event_id, updated_event_id FROM currency_fragments WHERE $1 OR status = 'active' ORDER BY code, id",
        )
        .bind(include_inactive)
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;
        let binding_rows = sqlx::query_as::<_, BindingRow>(
            "SELECT id, currency_id, chain_network, token_address FROM currency_token_binding_fragments ORDER BY currency_id, id",
        )
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(persistence)?;

        let items = rows
            .into_iter()
            .map(|row| {
                let status = match row.status.as_str() {
                    "defined" => CurrencyStatus::Defined,
                    "active" => CurrencyStatus::Active,
                    "inactive" => CurrencyStatus::Inactive,
                    _ => {
                        return Err(persistence(PgCurrencyListReaderError::Status(
                            row.status.clone(),
                        )));
                    }
                };
                let token_bindings = binding_rows
                    .iter()
                    .filter(|binding| binding.currency_id == row.id)
                    .map(|binding| {
                        Ok(CurrencyTokenBindingFragment {
                            id: TokenBindingId::try_from_uuid(binding.id).map_err(persistence)?,
                            chain_network: serde_json::from_str::<ChainNetwork>(
                                &binding.chain_network,
                            )
                            .map_err(persistence)?,
                            token_address: serde_json::from_str::<TokenAddress>(
                                &binding.token_address,
                            )
                            .map_err(persistence)?,
                        })
                    })
                    .collect::<Result<Vec<_>, CurrencyListReaderError>>()?;
                Ok(CurrencyFragment {
                    id: CurrencyId::try_from_uuid(row.id).map_err(persistence)?,
                    currency_registrar_id: CurrencyRegistrarId::try_from_uuid(
                        row.currency_registrar_id,
                    )
                    .map_err(persistence)?,
                    code: CurrencyCode::try_from(row.code).map_err(persistence)?,
                    decimals: CurrencyDecimals::new(
                        u8::try_from(row.decimals).map_err(persistence)?,
                    ),
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
                })
            })
            .collect::<Result<Vec<_>, CurrencyListReaderError>>()?;
        Ok(CurrencyList { items })
    }
}

fn persistence(error: impl std::error::Error + Send + Sync + 'static) -> CurrencyListReaderError {
    CurrencyListReaderError::Persistence(Box::new(error))
}
